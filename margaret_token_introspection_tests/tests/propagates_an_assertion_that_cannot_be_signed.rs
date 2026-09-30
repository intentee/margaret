use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client_error::AuthorizationServerClientError;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::oauth_client::OAuthClient;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;

#[tokio::test]
async fn propagates_an_assertion_that_cannot_be_signed() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/introspect",
        Arc::new(StaticHandler {
            body: Vec::new(),
            content_type: "application/json",
            status: 500,
        }),
    )
    .await;
    let admission = introspect_bearer_token::<RepositoryClaims>(
        &RequestAuthorization::parse(Some("Bearer opaque-token")),
        &server.client(Arc::new(OAuthClient {
            authentication: ClientAuthentication::PrivateKeyJwt(Arc::new(unrolled_store())),
            client_id: "client"
                .parse()
                .expect("the fixture client identifier is visible"),
        })),
    )
    .await;

    server.stop().await;

    assert!(matches!(
        admission,
        Err(AuthorizationServerClientError::AssertionSigning(_))
    ));
}
