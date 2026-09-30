use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use margaret_authorization_server_client::authorization_server_client_error::AuthorizationServerClientError;
use margaret_authorization_server_client_tests::counting_token_handler::CountingTokenHandler;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_client_credentials_tests::artifact_store_target::artifact_store_target;
use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::oauth_client::OAuthClient;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn propagates_an_assertion_that_cannot_be_signed() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(CountingTokenHandler {
            expires_in: Some(3600),
            issued: AtomicUsize::new(0),
        }),
    )
    .await;

    let acquired = ClientCredentials::create(Arc::new(
        server.client(Arc::new(OAuthClient {
            authentication: ClientAuthentication::PrivateKeyJwt(Arc::new(unrolled_store())),
            client_id: "client"
                .parse()
                .expect("the fixture client identifier is visible"),
        })),
    ))
    .access_token(&artifact_store_target())
    .await;

    server.stop().await;

    assert!(matches!(
        acquired.as_ref().map_err(AsRef::as_ref),
        Err(AuthorizationServerClientError::AssertionSigning(_))
    ));
}
