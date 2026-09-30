use std::sync::Arc;

use serde_json::Value;
use serde_json::json;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_client_credentials_tests::artifact_store_target::artifact_store_target;
use margaret_http::body_limit::BodyLimit;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn requests_a_client_credentials_grant_for_its_target() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(FormEchoHandler {
            limit: BodyLimit::new(1024),
            wrapping: EchoWrapping::AccessToken,
        }),
    )
    .await;
    let acquired =
        ClientCredentials::create(Arc::new(server.client(Arc::new(secret_basic_client()))))
            .access_token(&artifact_store_target())
            .await
            .expect("a secret basic client needs no assertion");

    server.stop().await;

    let AcquiredToken::Acquired(token) = acquired else {
        panic!("the token is acquired");
    };
    let echoed: Value = serde_json::from_str(token.secret()).expect("the echo is json");

    assert_eq!(
        echoed["fields"],
        json!({
            "grant_type": "client_credentials",
            "resource": "https://artifacts.example/",
            "scope": "artifacts:write",
        })
    );
}
