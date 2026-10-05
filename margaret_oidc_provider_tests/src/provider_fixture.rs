use std::sync::Arc;

use reqwest::Client;
use serde_json::Value;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::declares_accepted_client::DeclaresAcceptedClient;
use margaret_accepted_clients_tests::accepted_client_declaration::AcceptedClientDeclaration;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_oidc_provider::authorization_endpoint::AuthorizationEndpoint;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_endpoint::ConsentEndpoint;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider::provider_endpoints::ProviderEndpoints;
use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;
use margaret_token_signer_tests::token_issuance_declaration::TokenIssuanceDeclaration;

use crate::answer::Answer;
use crate::client_credentials::ClientCredentials;
use crate::fixture_endpoint_paths::FIXTURE_ENDPOINT_PATHS;
use crate::portal_client::portal_client;
use crate::provider_issuance::provider_issuance;
use crate::provider_parts::ProviderParts;
use crate::provider_url::provider_url;
use crate::service_client::service_client;
use crate::signed_in_end_user::signed_in_end_user;
use crate::spa_client::spa_client;
use crate::validated_form::validated_form;

fn fixture_clients() -> Vec<AcceptedClient> {
    vec![portal_client(), service_client(), spa_client()]
}

pub struct ProviderFixture {
    pub authorization: AuthorizationEndpoint,
    pub client: Client,
    pub consent: ConsentEndpoint,
    pub server: RunningFixtureServer,
    pub tls: TlsFixture,
}

impl ProviderFixture {
    async fn assembled(
        clients: Vec<AcceptedClient>,
        exchangers: Vec<Arc<SubjectTokenExchanger>>,
        state: Arc<dyn StoresProviderState>,
    ) -> Self {
        let issuance: Arc<dyn DeclaresTokenIssuance> = Arc::new(TokenIssuanceDeclaration {
            issuance: provider_issuance(),
        });
        let roller = fixture_roller();
        let secret_store = Arc::new(JwksSecretStore::create(
            Arc::clone(&roller),
            Arc::clone(&issuance),
        ));
        let clients = Arc::new(
            AcceptedClients::create(
                clients
                    .into_iter()
                    .map(|client| {
                        Arc::new(AcceptedClientDeclaration { client })
                            as Arc<dyn DeclaresAcceptedClient>
                    })
                    .collect(),
                issuance.as_ref(),
            )
            .expect("the fixture clients are accepted"),
        );
        let endpoints = Arc::new(
            ProviderEndpoints::create(issuance.as_ref(), FIXTURE_ENDPOINT_PATHS)
                .expect("the discovery route is served at the issuer"),
        );
        let parts = ProviderParts {
            clients,
            endpoints,
            issuance,
            roller,
            secret_store,
            state,
        };
        let tls = TlsFixture::generate();
        let server =
            RunningFixtureServer::start(tls.server_config.clone(), parts.routes(exchangers)).await;
        let client = fixture_client_builder(&tls.certificate_authority)
            .resolve(&tls.server_name, server.address())
            .build()
            .expect("the fixture client builds");

        Self {
            authorization: AuthorizationEndpoint::create(
                parts.clients,
                Arc::clone(&parts.state),
                parts.endpoints,
                Arc::clone(&parts.issuance),
            ),
            client,
            consent: ConsentEndpoint::create(parts.state, parts.issuance),
            server,
            tls,
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture clients, endpoints or tls client cannot be prepared.
    pub async fn over_state(state: Arc<dyn StoresProviderState>) -> Self {
        Self::assembled(fixture_clients(), Vec::new(), state).await
    }

    /// # Panics
    ///
    /// Panics when the fixture clients, endpoints or tls client cannot be prepared.
    pub async fn serving(
        clients: Vec<AcceptedClient>,
        exchangers: Vec<Arc<SubjectTokenExchanger>>,
    ) -> Self {
        Self::assembled(clients, exchangers, Arc::new(MemoryProviderState::create())).await
    }

    /// # Panics
    ///
    /// Panics when the fixture clients, endpoints or tls client cannot be prepared.
    pub async fn start(exchangers: Vec<Arc<SubjectTokenExchanger>>) -> Self {
        Self::serving(fixture_clients(), exchangers).await
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
        Answer::of(
            credentials
                .presented_on(self.client.post(provider_url(path)))
                .form(form)
                .send()
                .await
                .expect("the provider answers"),
        )
        .await
    }

    pub async fn stop(self) {
        self.server.stop().await;
    }
}
