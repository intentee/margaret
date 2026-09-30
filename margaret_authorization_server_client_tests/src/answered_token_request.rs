use std::collections::BTreeSet;
use std::sync::Arc;

use oauth2::basic::BasicTokenResponse;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

use crate::fixture_authorization_server::FixtureAuthorizationServer;
use crate::secret_basic_client::secret_basic_client;

/// # Panics
///
/// Panics when the token request of a secret basic client fails to be sent.
pub async fn answered_token_request(
    status: u16,
    body: Vec<u8>,
) -> EndpointOutcome<BasicTokenResponse> {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(StaticHandler {
            body,
            content_type: "application/json",
            status,
        }),
    )
    .await;
    let outcome = server
        .client(Arc::new(secret_basic_client()))
        .client_credentials(&TokenTarget {
            audience: TargetAudience::Unspecified,
            scopes: BTreeSet::new(),
        })
        .await
        .expect("a secret basic client needs no assertion");

    server.stop().await;

    outcome
}
