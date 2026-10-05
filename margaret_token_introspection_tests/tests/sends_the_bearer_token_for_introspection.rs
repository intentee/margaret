use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::token_admission::TokenAdmission;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_route_method::content_method::ContentMethod;
use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_token_introspection::introspected_token::IntrospectedToken;

#[derive(Deserialize)]
struct FormEcho {
    fields: HashMap<String, String>,
}

#[derive(Deserialize)]
struct EchoClaims {
    echo: FormEcho,
}

#[tokio::test]
async fn sends_the_bearer_token_for_introspection() {
    let server = FixtureAuthorizationServer::start(
        "/introspect",
        MethodHandler::content(
            ContentMethod::Post,
            Arc::new(FormEchoHandler {
                limit: BodyLimit::new(1024),
                wrapping: EchoWrapping::ActiveIntrospection,
            }),
        ),
    )
    .await;
    let admission = introspect_bearer_token::<EchoClaims>(
        &RequestAuthorization::parse(Some("Bearer opaque-token")),
        &server.client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        })),
    )
    .await;

    server.stop().await;

    let TokenAdmission::Admitted(IntrospectedToken { claims, .. }) = admission else {
        panic!("the echoed introspection is admitted");
    };

    assert_eq!(
        claims.echo.fields,
        HashMap::from([
            ("token".to_string(), "opaque-token".to_string()),
            ("token_type_hint".to_string(), "access_token".to_string()),
        ])
    );
}
