use std::sync::Arc;

use oauth2::EmptyExtraTokenFields;
use oauth2::StandardTokenResponse;
use oauth2::basic::BasicTokenType;
use serde_json::Value;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_route_method::route_method::RouteMethod;

use crate::fixture_authorization_server::FixtureAuthorizationServer;
use crate::oauth_client_declaration::OAuthClientDeclaration;
use crate::secret_basic_client::secret_basic_client;

pub async fn answered_grant(
    status: u16,
    body: &Value,
) -> EndpointOutcome<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
    let server = FixtureAuthorizationServer::start(
        "/token",
        MethodHandler::head(
            RouteMethod::Post,
            Arc::new(StaticHandler {
                body: body.to_string().into_bytes(),
                content_type: "application/json",
                status,
            }),
        ),
    )
    .await;
    let outcome = server
        .client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        }))
        .request_grant::<EmptyExtraTokenFields>(GrantType::TokenExchange, Vec::new())
        .await;

    server.stop().await;

    outcome
}
