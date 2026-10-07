use std::collections::BTreeSet;
use std::sync::Arc;

use oauth2::basic::BasicTokenResponse;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

use crate::fixture_authorization_server::FixtureAuthorizationServer;
use crate::secret_basic_authentication::secret_basic_authentication;

pub async fn answered_token_request(answer: StaticHandler) -> EndpointOutcome<BasicTokenResponse> {
    let server = FixtureAuthorizationServer::start(
        "/token",
        MethodHandler::head(RouteMethod::Post, Arc::new(answer)),
    )
    .await;
    let outcome = server
        .client(secret_basic_authentication())
        .client_credentials(&TokenTarget {
            audience: TargetAudience::Unspecified,
            scopes: BTreeSet::new(),
        })
        .await;

    server.stop().await;

    outcome
}
