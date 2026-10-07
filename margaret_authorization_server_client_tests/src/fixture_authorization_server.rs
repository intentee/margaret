use std::sync::Arc;

use url::Url;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_oidc_discovery::authorization_response_issuer::AuthorizationResponseIssuer;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::fixture_client_id::FIXTURE_CLIENT_ID;
use crate::localhost_discovery_metadata::localhost_discovery_metadata;
use crate::localhost_trust::localhost_trust;

pub struct FixtureAuthorizationServer {
    fixture: TlsFixture,
    server: RunningFixtureServer,
}

impl FixtureAuthorizationServer {
    pub async fn start(endpoint_path: &'static str, endpoint: MethodHandler) -> Self {
        let fixture = TlsFixture::generate();
        let server = RunningFixtureServer::start(
            fixture.server_config.clone(),
            vec![RouteEntry::new(endpoint_path, vec![endpoint])],
        )
        .await;

        Self { fixture, server }
    }

    /// # Panics
    ///
    /// Panics when the fixture request client cannot be built.
    #[must_use]
    pub fn client(&self, authentication: ClientAuthentication) -> AuthorizationServerClient {
        self.client_verifying_with(authentication, Arc::new(IssuerKeySet::awaiting()))
    }

    /// # Panics
    ///
    /// Panics when the fixture request client cannot be built.
    #[must_use]
    pub fn client_verifying_with(
        &self,
        authentication: ClientAuthentication,
        key_set: Arc<IssuerKeySet>,
    ) -> AuthorizationServerClient {
        let metadata = Arc::new(IssuerMetadata::awaiting());

        metadata.hold(localhost_discovery_metadata(
            AuthorizationResponseIssuer::Unadvertised,
            AdvertisedEndpoint::Advertised(
                Url::parse("https://localhost/introspect")
                    .expect("the localhost introspection endpoint is a url"),
            ),
        ));

        let trusted_issuer = Arc::new(TrustedIssuer::create(key_set, localhost_trust()));

        match authentication {
            ClientAuthentication::ClientSecretBasic(secret) => {
                AuthorizationServerClient::with_client_secret_basic(
                    self.request_client(),
                    metadata,
                    trusted_issuer,
                    FIXTURE_CLIENT_ID,
                    secret,
                )
            }
            ClientAuthentication::PrivateKeyJwt(roller) => {
                AuthorizationServerClient::with_private_key_jwt(
                    self.request_client(),
                    metadata,
                    trusted_issuer,
                    FIXTURE_CLIENT_ID,
                    roller,
                )
            }
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture request client cannot be built.
    #[must_use]
    pub fn request_client(&self) -> Arc<IssuerRequestClient> {
        Arc::new(
            IssuerRequestClient::build(
                fixture_client_builder(&self.fixture.certificate_authority)
                    .resolve(&self.fixture.server_name, self.server.address()),
            )
            .expect("the fixture request client builds"),
        )
    }

    pub async fn stop(self) {
        self.server.stop().await;
    }
}
