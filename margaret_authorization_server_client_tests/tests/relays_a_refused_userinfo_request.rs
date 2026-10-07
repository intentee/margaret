use std::sync::Arc;

use http::StatusCode;
use oauth2::AccessToken;
use serde_json::Value;

use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn relays_a_refused_userinfo_request() {
    let server = FixtureAuthorizationServer::start(
        "/userinfo",
        MethodHandler::head(
            RouteMethod::Get,
            Arc::new(StaticHandler {
                body: Vec::new(),
                content_type: "application/json",
                status: 401,
            }),
        ),
    )
    .await;

    let outcome = server
        .client(secret_basic_authentication())
        .userinfo::<Value>(&AccessToken::new("expired".to_string()))
        .await;

    server.stop().await;

    assert!(matches!(
        outcome,
        UserinfoOutcome::Refused { status } if status == StatusCode::UNAUTHORIZED
    ));
}
