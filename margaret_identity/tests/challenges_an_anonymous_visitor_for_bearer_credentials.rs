use std::sync::Arc;

use async_trait::async_trait;
use reqwest::header::AUTHORIZATION;
use reqwest::header::HeaderValue;
use reqwest::header::WWW_AUTHENTICATE;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret_identity::require_bearer_authenticated_user::require_bearer_authenticated_user;

struct BearerGreeting;

#[async_trait]
impl Handler for BearerGreeting {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        let outcome = match request.inputs.server.authorization() {
            RequestAuthorization::Bearer(token) => {
                AuthenticatedUserOutcome::Authenticated(token.as_str().to_string())
            }
            RequestAuthorization::Absent
            | RequestAuthorization::Malformed
            | RequestAuthorization::OtherScheme => AuthenticatedUserOutcome::Anonymous,
        };

        Ok(match require_bearer_authenticated_user(outcome) {
            Ok(runner) => ResponseContinuation::Done(Response::text(200, runner)),
            Err(continuation) => continuation,
        })
    }
}

#[tokio::test]
async fn challenges_an_anonymous_visitor_for_bearer_credentials() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/",
            vec![MethodHandler::anonymous("GET", Arc::new(BearerGreeting))],
        )],
    )
    .await;
    let client = fixture_client_builder(&fixture.certificate_authority)
        .build()
        .expect("the fixture client builds");

    let response = client
        .get(fixture.url(server.port(), "/"))
        .header(AUTHORIZATION, "Basic cnVubmVyOnNlY3JldA==")
        .send()
        .await
        .expect("the fixture server answers");

    assert_eq!(response.status().as_u16(), 401);
    assert_eq!(
        response.headers().get(WWW_AUTHENTICATE),
        Some(&HeaderValue::from_static("Bearer"))
    );

    server.stop().await;
}
