use std::sync::Arc;
use std::sync::OnceLock;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_oauth_client::oauth_client::OAuthClient;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;

use crate::token_endpoint::TokenEndpoint;

pub struct SignInFixture {
    pub flow: SignInFlow,
    pub issuer_secret: JwksSecret,
    pub secret_store: Arc<JwksSecretStore>,
    pub server: FixtureAuthorizationServer,
    pub token_endpoint: Arc<TokenEndpoint>,
}

impl SignInFixture {
    pub async fn start(declaration: OAuthClient) -> Self {
        let token_endpoint = Arc::new(TokenEndpoint {
            answer: OnceLock::new(),
        });
        let server =
            FixtureAuthorizationServer::start(RouteMethod::Post, "/token", token_endpoint.clone())
                .await;
        let issuer_secret = fresh_p256_secret();
        let client = server.client(Arc::new(declaration));
        let secret_store = Arc::new(rolled_store(fresh_p256_secret()));

        client
            .trusted_issuer
            .key_set
            .hold(issuer_secret.key_set().clone());

        Self {
            flow: SignInFlow::create(Arc::new(client), Arc::clone(&secret_store)),
            issuer_secret,
            secret_store,
            server,
            token_endpoint,
        }
    }
}
