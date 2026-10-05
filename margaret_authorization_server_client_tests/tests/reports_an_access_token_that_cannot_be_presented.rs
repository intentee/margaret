use std::sync::Arc;

use oauth2::AccessToken;
use serde_json::Value;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
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
        .client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        }))
        .userinfo::<Value>(&AccessToken::new("line\nbreak".to_string()))
        .await;

    server.stop().await;

    assert!(matches!(
        outcome,
        UserinfoOutcome::Unavailable(ServerUnavailability::AccessTokenUnpresentable { .. })
    ));
}
