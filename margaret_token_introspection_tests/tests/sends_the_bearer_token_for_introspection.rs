use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::body_limit::BodyLimit;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_token_introspection::introspected_token::IntrospectedToken;
use margaret_token_introspection::introspection_admission::IntrospectionAdmission;

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
        RouteMethod::Post,
        "/introspect",
        Arc::new(FormEchoHandler {
            limit: BodyLimit::new(1024),
            wrapping: EchoWrapping::ActiveIntrospection,
        }),
    )
    .await;
    let admission = introspect_bearer_token::<EchoClaims>(
        &RequestAuthorization::parse(Some("Bearer opaque-token")),
        &server.client(Arc::new(secret_basic_client())),
    )
    .await
    .expect("a secret basic client needs no assertion");

    server.stop().await;

    let IntrospectionAdmission::Admitted(IntrospectedToken { claims, .. }) = admission else {
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
