use std::sync::Arc;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::token_admission::TokenAdmission;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;

#[tokio::test]
async fn refuses_malformed_credentials_as_an_invalid_request() {
    let server = FixtureAuthorizationServer::start(
        "/introspect",
        MethodHandler::head(
            RouteMethod::Post,
            Arc::new(StaticHandler {
                body: Vec::new(),
                content_type: "application/json",
                status: 500,
            }),
        ),
    )
    .await;
    let admission = introspect_bearer_token::<RepositoryClaims>(
        &RequestAuthorization::parse(Some("Bearer")),
        &server.client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        })),
    )
    .await;

    server.stop().await;

    assert!(matches!(
        admission,
        TokenAdmission::Refused(ResponseContinuation::Done(response)) if response.status() == 400
    ));
}
