use std::sync::Arc;

use oauth2::EmptyExtraTokenFields;
use oauth2::TokenResponse;
use serde_json::Value;
use serde_json::json;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::form_parameter::FormParameter;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_route_method::content_method::ContentMethod;

#[tokio::test]
async fn authenticates_a_grant_with_client_secret_basic() {
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

    let outcome = server
        .client(secret_basic_authentication())
        .request_grant::<EmptyExtraTokenFields>(
            GrantType::TokenExchange,
            vec![FormParameter {
                name: "subject_token",
                value: "grant.subject.token".to_string(),
            }],
        )
        .await;

    server.stop().await;

    let EndpointOutcome::Answered(response) = outcome else {
        panic!("the token endpoint answers");
    };
    let echoed: Value =
        serde_json::from_str(response.access_token().secret()).expect("the echo is json");

    assert_eq!(
        echoed,
        json!({
            "authorization": "Basic Y2xpZW50JTNBaWQ6czNjcmV0JTJGJTJCJTNE",
            "fields": {
                "grant_type": "urn:ietf:params:oauth:grant-type:token-exchange",
                "subject_token": "grant.subject.token",
            },
        })
    );
}
