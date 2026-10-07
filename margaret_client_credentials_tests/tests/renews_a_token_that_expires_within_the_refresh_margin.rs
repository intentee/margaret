use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use margaret_authorization_server_client_tests::counting_token_handler::CountingTokenHandler;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::acquired_token_refresh_margin::ACQUIRED_TOKEN_REFRESH_MARGIN;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_client_credentials_tests::artifact_store_target::artifact_store_target;
use margaret_http::method_handler::MethodHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn renews_a_token_that_expires_within_the_refresh_margin() {
    let handler = Arc::new(CountingTokenHandler {
        expires_in: Some(ACQUIRED_TOKEN_REFRESH_MARGIN.as_secs()),
        issued: AtomicUsize::new(0),
    });
    let server = FixtureAuthorizationServer::start(
        "/token",
        MethodHandler::head(RouteMethod::Post, Arc::clone(&handler) as _),
    )
    .await;
    let client_credentials =
        ClientCredentials::create(Arc::new(server.client(secret_basic_authentication())));

    client_credentials
        .access_token(&artifact_store_target())
        .await;

    let renewed = client_credentials
        .access_token(&artifact_store_target())
        .await;

    server.stop().await;

    assert!(matches!(renewed, AcquiredToken::Acquired(token) if token.secret() == "token-2"));
    assert_eq!(handler.issued.load(Ordering::SeqCst), 2);
}
