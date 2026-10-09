use std::sync::Arc;

use serde_json::Value;
use serde_json::json;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::resource_credentials::ResourceCredentials;
use margaret_client_credentials::resource_grant::ResourceGrant;
use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_route_method::content_method::ContentMethod;

#[tokio::test]
async fn requests_the_declared_grant_of_its_resource() {
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
    let acquired = ResourceCredentials::create(
        Arc::new(server.client(secret_basic_authentication())),
        ResourceGrant {
            audience: "attachments",
            scopes: &["files:write", "files:read"],
        },
    )
    .expect("the grant scopes are scope tokens")
    .access_token()
    .await;

    server.stop().await;

    let AcquiredToken::Acquired(token) = acquired else {
        panic!("the token is acquired");
    };
    let echoed: Value = serde_json::from_str(token.secret()).expect("the echo is json");

    assert_eq!(
        echoed["fields"],
        json!({
            "audience": "attachments",
            "grant_type": "client_credentials",
            "scope": "files:read files:write",
        })
    );
}
