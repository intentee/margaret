use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::sync::Arc;

use form_urlencoded::Serializer;
use http::header::CONTENT_TYPE;
use reqwest::Client;
use serde_json::Value;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients_tests::assertion_claims::assertion_claims;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_oauth_vocabulary::jwt_bearer_client_assertion_type::JWT_BEARER_CLIENT_ASSERTION_TYPE;
use margaret_oidc_provider::authorization_endpoint::AuthorizationEndpoint;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_endpoint::ConsentEndpoint;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::answer::Answer;
use crate::client_credentials::ClientCredentials;
use crate::fixture_authorization_grants::FixtureAuthorizationGrants;
use crate::fixture_clients::FixtureClients;
use crate::fixture_endpoints::fixture_endpoints;
use crate::grant_interference::GrantInterference;
use crate::provider_issuance::provider_issuance;
use crate::provider_parts::ProviderParts;
use crate::provider_url::provider_url;
use crate::published_endpoints::PublishedEndpoints;
use crate::signed_in_end_user::signed_in_end_user;
use crate::validated_form::validated_form;

const FORM_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

struct ProviderPublication {
    endpoints: PublishedEndpoints,
    ip: IpAddr,
    issuance: TokenIssuance,
    tls: TlsFixture,
}

impl ProviderPublication {
    fn on_localhost() -> Self {
        Self {
            endpoints: fixture_endpoints(
                &provider_issuance()
                    .issuer
                    .parse()
                    .expect("the fixture issuer is an https url"),
            ),
            ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
            issuance: provider_issuance(),
            tls: TlsFixture::generate(),
        }
    }
}

pub struct ProviderFixture {
    pub authorization: AuthorizationEndpoint,
    pub client: Client,
    pub clients: FixtureClients,
    pub consent: ConsentEndpoint,
    pub issuer: &'static str,
    pub server: RunningFixtureServer,
    pub tls: TlsFixture,
}

impl ProviderFixture {
    async fn assembled(
        clients: FixtureClients,
        exchangers: Vec<Arc<SubjectTokenExchanger>>,
        ProviderPublication {
            endpoints:
                PublishedEndpoints {
                    authorization,
                    provider: endpoints,
                },
            ip,
            issuance,
            tls,
        }: ProviderPublication,
    ) -> Self {
        let roller = fixture_roller().await;
        let secret_store = Arc::new(JwksSecretStore::create(Arc::clone(&roller), issuance));
        let parts = ProviderParts {
            clients: Arc::new(AcceptedClients::create(clients.registered(), issuance)),
            endpoints,
            grants: Arc::clone(&clients.grants),
            issuance,
            roller,
            secret_store,
        };
        let server =
            RunningFixtureServer::start_at(ip, tls.server_config.clone(), parts.routes(exchangers))
                .await;
        let client = fixture_client_builder(&tls.certificate_authority)
            .resolve(&tls.server_name, server.address())
            .build()
            .expect("the fixture client builds");

        Self {
            authorization: AuthorizationEndpoint::create(
                parts.clients,
                authorization,
                parts.issuance,
            ),
            client,
            consent: ConsentEndpoint::create(Arc::clone(&clients.grants), parts.issuance),
            clients,
            issuer: parts.issuance.issuer,
            server,
            tls,
        }
    }

    /// # Panics
    ///
    /// Panics when the issuer names no host, or the fixture clients, endpoints or tls client
    /// cannot be prepared.
    pub async fn published(issuer: IssuerIdentifier, ip: IpAddr) -> Self {
        let tls = TlsFixture::serving(issuer.url().host_str().expect("the issuer names a host"));

        Self::assembled(
            FixtureClients::standard().await,
            Vec::new(),
            ProviderPublication {
                endpoints: fixture_endpoints(&issuer),
                ip,
                issuance: TokenIssuance {
                    issuer: String::leak(issuer.as_str().to_string()),
                    ..provider_issuance()
                },
                tls,
            },
        )
        .await
    }

    /// # Panics
    ///
    /// Panics when the fixture clients, endpoints or tls client cannot be prepared.
    pub async fn interfered(interference: GrantInterference) -> Self {
        Self::serving(
            FixtureClients::over_grants(Arc::new(FixtureAuthorizationGrants::interfered(
                interference,
            )))
            .await,
            Vec::new(),
        )
        .await
    }

    /// # Panics
    ///
    /// Panics when the fixture clients, endpoints or tls client cannot be prepared.
    pub async fn serving(
        clients: FixtureClients,
        exchangers: Vec<Arc<SubjectTokenExchanger>>,
    ) -> Self {
        Self::assembled(clients, exchangers, ProviderPublication::on_localhost()).await
    }

    /// # Panics
    ///
    /// Panics when the fixture clients, endpoints or tls client cannot be prepared.
    pub async fn start(exchangers: Vec<Arc<SubjectTokenExchanger>>) -> Self {
        Self::serving(FixtureClients::standard().await, exchangers).await
    }

    /// # Panics
    ///
    /// Panics when the authorization cannot reach the provider state.
    pub async fn authorized(&self, parameters: &Value) -> AuthorizationOutcome {
        self.authorization
            .authorize(
                validated_form(parameters),
                &EndUserAuthentication::Authenticated(signed_in_end_user()),
            )
            .await
            .expect("the authorization reaches its state")
    }

    /// # Panics
    ///
    /// Panics when no asserting fixture client carries the client identifier.
    #[must_use]
    pub fn assertion(&self, client_id: &'static str) -> String {
        self.clients
            .asserting(client_id)
            .assertion(&assertion_claims(client_id, self.issuer))
    }

    /// # Panics
    ///
    /// Panics when the provider does not answer.
    pub async fn get(&self, path: &str) -> Answer {
        Answer::of(
            self.client
                .get(provider_url(path))
                .send()
                .await
                .expect("the provider answers"),
        )
        .await
    }

    /// # Panics
    ///
    /// Panics when the issuer request client cannot be built.
    #[must_use]
    pub fn issuer_request_client(&self) -> IssuerRequestClient {
        IssuerRequestClient::build(
            fixture_client_builder(&self.tls.certificate_authority)
                .resolve(&self.tls.server_name, self.server.address()),
        )
        .expect("the issuer request client builds")
    }

    /// # Panics
    ///
    /// Panics when the provider does not answer.
    pub async fn post_form(
        &self,
        path: &str,
        credentials: &ClientCredentials,
        form: &Value,
    ) -> Answer {
        let mut form = form.clone();
        let request = self.client.post(provider_url(path));
        let request = match self.presented_assertion(credentials) {
            Some(assertion) => {
                form["client_assertion"] = Value::String(assertion);
                form["client_assertion_type"] =
                    Value::String(JWT_BEARER_CLIENT_ASSERTION_TYPE.to_string());

                request
            }
            None => credentials.presented_on(request),
        };

        Answer::of(
            request
                .form(&form)
                .send()
                .await
                .expect("the provider answers"),
        )
        .await
    }

    /// # Panics
    ///
    /// Panics when the provider does not answer.
    pub async fn post_encoded_form(
        &self,
        path: &str,
        client_id: &'static str,
        form: &str,
    ) -> Answer {
        let presented = Serializer::new(String::new())
            .append_pair("client_assertion", &self.assertion(client_id))
            .append_pair("client_assertion_type", JWT_BEARER_CLIENT_ASSERTION_TYPE)
            .finish();

        Answer::of(
            self.client
                .post(provider_url(path))
                .header(CONTENT_TYPE, FORM_CONTENT_TYPE)
                .body(format!("{form}&{presented}"))
                .send()
                .await
                .expect("the provider answers"),
        )
        .await
    }

    pub async fn stop(self) {
        self.server.stop().await;
    }

    fn presented_assertion(&self, credentials: &ClientCredentials) -> Option<String> {
        match credentials {
            ClientCredentials::Asserted { client_id } => Some(self.assertion(client_id)),
            ClientCredentials::Assertion(assertion) => Some(assertion.clone()),
            ClientCredentials::Absent
            | ClientCredentials::Basic { .. }
            | ClientCredentials::Bearer(_) => None,
        }
    }
}
