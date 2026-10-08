use std::sync::Arc;

use oauth2::AccessToken;
use serde_json::Value;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_issuer_request::issuer_response_max_bytes::ISSUER_RESPONSE_MAX_BYTES;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn reports_oversized_userinfo() {
    let server = FixtureAuthorizationServer::start(
        "/userinfo",
        MethodHandler::head(
            RouteMethod::Get,
            Arc::new(StaticHandler {
                body: vec![b' '; ISSUER_RESPONSE_MAX_BYTES + 1],
                content_type: "application/json",
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
        UserinfoOutcome::Unavailable(ServerUnavailability::OversizedAnswer {
            max_bytes: ISSUER_RESPONSE_MAX_BYTES
        })
    ));
}
