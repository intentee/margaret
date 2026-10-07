use std::sync::Arc;
use std::sync::OnceLock;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_http::method_handler::MethodHandler;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;

use crate::token_endpoint::TokenEndpoint;

pub struct SignInFixture {
    pub flow: SignInFlow,
    pub issuer_secret: JwksSecret,
    pub roller: Arc<JwksRoller>,
    pub server: FixtureAuthorizationServer,
    pub token_endpoint: Arc<TokenEndpoint>,
}

impl SignInFixture {
    pub async fn start(authentication: ClientAuthentication) -> Self {
        let token_endpoint = Arc::new(TokenEndpoint {
            answer: OnceLock::new(),
        });
        let server = FixtureAuthorizationServer::start(
            "/token",
            MethodHandler::head(RouteMethod::Post, token_endpoint.clone()),
        )
        .await;
        let issuer_secret = fresh_p256_secret();
        let key_set = Arc::new(IssuerKeySet::awaiting());

        key_set.hold(Arc::new(issuer_secret.key_set().clone()));

        let client = server.client_verifying_with(authentication, key_set);
        let roller = fixture_roller();

        Self {
            flow: SignInFlow::create(Arc::new(client), Arc::clone(&roller)),
            issuer_secret,
            roller,
            server,
            token_endpoint,
        }
    }
}
