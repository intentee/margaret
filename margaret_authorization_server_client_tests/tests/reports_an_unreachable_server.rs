use std::collections::BTreeSet;
use std::sync::Arc;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn reports_an_unreachable_server() {
    let server = FixtureAuthorizationServer::start(
        "/token",
        MethodHandler::head(
            RouteMethod::Post,
            Arc::new(StaticHandler {
                body: Vec::new(),
                content_type: "application/json",
                status: 200,
            }),
        ),
    )
    .await;
    let client = server.client(Arc::new(OAuthClientDeclaration {
        client: secret_basic_client(),
    }));

    server.stop().await;

    assert!(matches!(
        client
            .client_credentials(&TokenTarget {
                audience: TargetAudience::Unspecified,
                scopes: BTreeSet::new(),
            })
            .await,
        EndpointOutcome::Unavailable(ServerUnavailability::Exchange(IssuerExchangeError::Transport(error))) if error.is_connect()
    ));
}
