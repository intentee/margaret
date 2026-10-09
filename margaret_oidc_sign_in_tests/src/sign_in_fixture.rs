use std::sync::Arc;
use std::sync::OnceLock;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_http::method_handler::MethodHandler;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::fixture_secrets::fixture_secrets;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_route_method::route_method::RouteMethod;

use crate::fixture_sign_in_flow::fixture_sign_in_flow;
use crate::token_endpoint::TokenEndpoint;

pub struct SignInFixture {
    pub client: Arc<AuthorizationServerClient>,
    pub flow: Arc<SignInFlow>,
    pub issuer_secret: JwksSecret,
    pub secrets: Arc<JwksSecretHolder>,
    pub server: FixtureAuthorizationServer,
    pub token_endpoint: Arc<TokenEndpoint>,
}

impl SignInFixture {
    pub async fn start(authentication: ClientAuthentication) -> Self {
        Self::with_secrets(authentication, fixture_secrets()).await
    }

    pub async fn with_secrets(
        authentication: ClientAuthentication,
        secrets: Arc<JwksSecretHolder>,
    ) -> Self {
        let token_endpoint = Arc::new(TokenEndpoint {
            answer: OnceLock::new(),
        });
        let server = FixtureAuthorizationServer::start(
            "/token",
            MethodHandler::head(RouteMethod::Post, token_endpoint.clone()),
        )
        .await;
        let issuer_secret = fresh_secret(SigningCurve::P256);
        let key_set = Arc::new(IssuerKeySet::awaiting());

        key_set.hold(Arc::new(issuer_secret.published_key_set().clone()));

        let client = Arc::new(server.client_verifying_with(authentication, key_set));

        Self {
            flow: fixture_sign_in_flow(Arc::clone(&client), Arc::clone(&secrets)),
            client,
            issuer_secret,
            secrets,
            server,
            token_endpoint,
        }
    }
}
