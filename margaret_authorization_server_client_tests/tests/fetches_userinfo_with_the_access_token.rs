use std::sync::Arc;

use oauth2::AccessToken;
use serde_json::Value;
use serde_json::json;

use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_authorization_server_client_tests::userinfo_echo_handler::UserinfoEchoHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn fetches_userinfo_with_the_access_token() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Get,
        "/userinfo",
        Arc::new(UserinfoEchoHandler),
    )
    .await;

    let outcome = server
        .client(Arc::new(secret_basic_client()))
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
