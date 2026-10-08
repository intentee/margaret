use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use chrono::Utc;
use rustls::ServerConfig;
use serde_json::json;

use margaret::framework::identity_session::id_token_lifetime_secs::ID_TOKEN_LIFETIME_SECS;
use margaret::framework::registered_claims::numeric_date::NumericDate;
use margaret_accepted_clients_tests::assertion_claims::assertion_claims;
use margaret_cluster_fixture::margaret::accepted_clients::clients::auth_accepted_partner_client_accepted_partner_client;
use margaret_cluster_fixture::margaret::token_issuance::TOKEN_ISSUANCE;
use margaret_cluster_fixture::margaret::trusted_issuers::external;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwk_pair::JwkPair;
use margaret_route_method::route_method::RouteMethod;
use margaret_sync_holder::sync_holder::SyncHolder;

use crate::declared_path::declared_path;
use crate::declared_port::declared_port;
use crate::fresh_key::fresh_key;
use crate::key_set_document::key_set_document;
use crate::published_documents::PublishedDocuments;
use crate::relay_destination::RelayDestination;
use crate::tls_relay::TlsRelay;

pub struct ExternalIssuer {
    external_jwks: SyncHolder<Bytes>,
    external_key: JwkPair,
    partner_key: JwkPair,
    relay: TlsRelay,
    server: RunningFixtureServer,
}

impl ExternalIssuer {
    pub async fn open(server_config: &Arc<ServerConfig>) -> Self {
        let external_key = fresh_key();
        let partner_key = fresh_key();
        let external_jwks = SyncHolder::new(key_set_document(&external_key));
        let mut documents = PublishedDocuments::default();

        documents.publish(
            declared_path(external::jwks_endpoint_issuer::JWKS_ENDPOINT_ISSUER.jwks_uri),
            external_jwks.clone(),
        );
        documents.publish(
            declared_path(
                auth_accepted_partner_client_accepted_partner_client::jwks_endpoint_issuer::JWKS_ENDPOINT_ISSUER
                    .jwks_uri,
            ),
            SyncHolder::new(key_set_document(&partner_key)),
        );

        let server = RunningFixtureServer::start_plain(
            UploadConfig::Disabled,
            vec![RouteEntry::new(
                "/{*document}",
                vec![MethodHandler::head(RouteMethod::Get, Arc::new(documents))],
            )],
        )
        .await;
        let relay = TlsRelay::open(
            SocketAddr::from((
                Ipv4Addr::LOCALHOST,
                declared_port(external::jwks_endpoint_issuer::JWKS_ENDPOINT_ISSUER.jwks_uri),
            )),
            Arc::clone(server_config),
            RelayDestination::Fixed(server.address()),
        )
        .await;

        Self {
            external_jwks,
            external_key,
            partner_key,
            relay,
            server,
        }
    }

    #[must_use]
    pub fn external_token(&self, subject: &str) -> String {
        let issued_at = NumericDate::from(Utc::now());

        self.external_key.sign_json(
            &json!({
                "aud": external::token_trust::TOKEN_TRUST.audience,
                "exp": issued_at
                    .after(Duration::from_secs(u64::from(ID_TOKEN_LIFETIME_SECS)))
                    .seconds_since_epoch(),
                "iat": issued_at.seconds_since_epoch(),
                "iss": external::token_trust::TOKEN_TRUST.issuer,
                "sub": subject,
            }),
            JwtType::Jwt,
        )
    }

    #[must_use]
    pub fn partner_assertion(&self) -> String {
        self.partner_key.sign_json(
            &assertion_claims(
                auth_accepted_partner_client_accepted_partner_client::accepted_client::ACCEPTED_CLIENT
                    .client_id,
                TOKEN_ISSUANCE.issuer,
            ),
            JwtType::ClientAuthentication,
        )
    }

    pub fn rotate_external_key(&mut self) {
        self.external_key = fresh_key();
        self.external_jwks.set(key_set_document(&self.external_key));
    }

    pub async fn close(self) {
        self.relay.close().await;
        self.server.stop().await;
    }
}
