use std::sync::Arc;

use oauth2::EmptyExtraTokenFields;
use oauth2::StandardTokenResponse;
use oauth2::basic::BasicTokenType;
use serde_json::Value;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

use crate::fixture_authorization_server::FixtureAuthorizationServer;
use crate::secret_basic_client::secret_basic_client;

/// # Panics
///
/// Panics when the grant request of a secret basic client fails to be sent.
pub async fn answered_grant(
    status: u16,
    body: &Value,
) -> EndpointOutcome<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(StaticHandler {
            body: body.to_string().into_bytes(),
            content_type: "application/json",
            status,
        }),
    )
    .await;
    let outcome = server
        .client(Arc::new(secret_basic_client()))
        .request_grant::<EmptyExtraTokenFields>(
            "urn:ietf:params:oauth:grant-type:jwt-bearer",
            Vec::new(),
        )
        .await
        .expect("a secret basic client needs no assertion");

    server.stop().await;

    outcome
}
