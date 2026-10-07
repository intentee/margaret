use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use margaret_authorization_server_client_tests::counting_token_handler::CountingTokenHandler;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_client_credentials_tests::artifact_store_target::artifact_store_target;
use margaret_http::method_handler::MethodHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn requests_one_token_for_concurrent_callers() {
    let handler = Arc::new(CountingTokenHandler {
        expires_in: Some(3600),
        issued: AtomicUsize::new(0),
    });
    let server = FixtureAuthorizationServer::start(
        "/token",
        MethodHandler::head(RouteMethod::Post, Arc::clone(&handler) as _),
    )
    .await;
    let client_credentials =
        ClientCredentials::create(Arc::new(server.client(secret_basic_authentication())));
    let target = artifact_store_target();

    let (first, second) = tokio::join!(
        client_credentials.access_token(&target),
        client_credentials.access_token(&target)
    );

    server.stop().await;

    assert!(matches!(first, AcquiredToken::Acquired(token) if token.secret() == "token-1"));
    assert!(matches!(second, AcquiredToken::Acquired(token) if token.secret() == "token-1"));
    assert_eq!(handler.issued.load(Ordering::SeqCst), 1);
}
