use std::sync::Arc;

use serde::de::DeserializeOwned;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::token_admission::TokenAdmission;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_token_introspection::introspected_token::IntrospectedToken;

pub async fn introspected_with_body<TClaims: DeserializeOwned>(
    status: u16,
    body: Vec<u8>,
) -> TokenAdmission<IntrospectedToken<TClaims>> {
    let server = FixtureAuthorizationServer::start(
        "/introspect",
        MethodHandler::head(
            RouteMethod::Post,
            Arc::new(StaticHandler {
                body,
                content_type: "application/json",
                status,
            }),
        ),
    )
    .await;
    let admission = introspect_bearer_token(
        &RequestAuthorization::parse(Some("Bearer opaque-token")),
        &server.client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        })),
    )
    .await;

    server.stop().await;

    admission
}
