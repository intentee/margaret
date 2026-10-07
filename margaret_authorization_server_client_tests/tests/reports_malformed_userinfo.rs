use std::sync::Arc;

use oauth2::AccessToken;
use serde_json::Value;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn reports_malformed_userinfo() {
    let server = FixtureAuthorizationServer::start(
        "/userinfo",
        MethodHandler::head(
            RouteMethod::Get,
            Arc::new(StaticHandler {
                body: b"eyJhbGciOiJSUzI1NiJ9.e30.c2ln".to_vec(),
                content_type: "application/jwt",
                status: 200,
            }),
        ),
    )
    .await;

    let outcome = server
        .client(secret_basic_authentication())
        .userinfo::<Value>(&AccessToken::new("token".to_string()))
        .await;

    server.stop().await;

    assert!(matches!(
        outcome,
        UserinfoOutcome::Unavailable(ServerUnavailability::MalformedAnswer { .. })
    ));
}
