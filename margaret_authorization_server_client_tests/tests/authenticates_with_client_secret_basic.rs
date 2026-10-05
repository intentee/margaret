use std::collections::BTreeSet;
use std::sync::Arc;

use oauth2::TokenResponse;
use serde_json::Value;
use serde_json::json;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_route_method::content_method::ContentMethod;

#[tokio::test]
async fn authenticates_with_client_secret_basic() {
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
        .client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        }))
        .client_credentials(&TokenTarget {
            audience: TargetAudience::Audience("artifact-store".to_string()),
            scopes: BTreeSet::new(),
        })
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
            "fields": { "audience": "artifact-store", "grant_type": "client_credentials" },
        })
    );
}
