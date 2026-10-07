use std::sync::Arc;

use oauth2::AccessToken;
use serde_json::Value;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_authorization_server_client_tests::userinfo_echo_handler::UserinfoEchoHandler;
use margaret_http::method_handler::MethodHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn reports_an_access_token_that_cannot_be_presented() {
    let server = FixtureAuthorizationServer::start(
        "/userinfo",
        MethodHandler::head(RouteMethod::Get, Arc::new(UserinfoEchoHandler)),
    )
    .await;

    let outcome = server
        .client(secret_basic_authentication())
        .userinfo::<Value>(&AccessToken::new("line\nbreak".to_string()))
        .await;

    server.stop().await;

    assert!(matches!(
        outcome,
        UserinfoOutcome::Unavailable(ServerUnavailability::AccessTokenUnpresentable { .. })
    ));
}
