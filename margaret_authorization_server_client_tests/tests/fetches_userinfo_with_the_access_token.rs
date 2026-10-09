use std::sync::Arc;

use oauth2::AccessToken;
use serde_json::Value;
use serde_json::json;

use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_authorization_server_client_tests::userinfo_echo_handler::UserinfoEchoHandler;
use margaret_http::method_handler::MethodHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn fetches_userinfo_with_the_access_token() {
    let server = FixtureAuthorizationServer::start(
        "/userinfo",
        MethodHandler::head(RouteMethod::Get, Arc::new(UserinfoEchoHandler)),
    )
    .await;

    let outcome = server
        .client(secret_basic_authentication())
        .userinfo::<Value>(&AccessToken::new("2YotnFZFEjr1zCsicMWpAA".to_string()))
        .await;

    server.stop().await;

    let UserinfoOutcome::Answered(userinfo) = outcome else {
        panic!("the userinfo endpoint answers");
    };

    assert_eq!(
        userinfo,
        json!({ "authorization": "Bearer 2YotnFZFEjr1zCsicMWpAA", "sub": "subject" })
    );
}
