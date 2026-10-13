use std::sync::Arc;

use serde_json::Value;
use serde_json::json;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_client_credentials_tests::artifact_store_target::artifact_store_target;
use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_route_method::content_method::ContentMethod;

#[tokio::test]
async fn requests_a_client_credentials_grant_for_its_target() {
    let server = FixtureAuthorizationServer::start(
        "/token",
        MethodHandler::content(
            ContentMethod::Post,
            Arc::new(FormEchoHandler {
                limit: BodyLimit::new(1024),
                wrapping: EchoWrapping::AccessToken,
            }),
        ),
    )
    .await;
    let acquired =
        ClientCredentials::create(Arc::new(server.client(secret_basic_authentication())))
            .access_token(&artifact_store_target())
            .await;

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
