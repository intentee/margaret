use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_authorization_server_client_tests::counting_token_handler::CountingTokenHandler;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_client_credentials_tests::artifact_store_target::artifact_store_target;
use margaret_http::method_handler::MethodHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn requests_a_token_per_distinct_target() {
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
        ClientCredentials::create(Arc::new(server.client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        }))));

    let artifacts = client_credentials
        .access_token(&artifact_store_target())
        .await;
    let unspecified = client_credentials
        .access_token(&TokenTarget {
            audience: TargetAudience::Unspecified,
            scopes: BTreeSet::new(),
        })
        .await;

    server.stop().await;

    assert!(matches!(artifacts, AcquiredToken::Acquired(token) if token.secret() == "token-1"));
    assert!(matches!(unspecified, AcquiredToken::Acquired(token) if token.secret() == "token-2"));
    assert_eq!(handler.issued.load(Ordering::SeqCst), 2);
}
