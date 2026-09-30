use std::sync::Arc;

use oauth2::EmptyExtraTokenFields;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn reports_an_unreachable_grant_endpoint() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(StaticHandler {
            body: Vec::new(),
            content_type: "application/json",
            status: 200,
        }),
    )
    .await;
    let client = server.client(Arc::new(secret_basic_client()));

    server.stop().await;

    assert!(matches!(
        client
            .request_grant::<EmptyExtraTokenFields>(
                "urn:ietf:params:oauth:grant-type:jwt-bearer",
                Vec::new(),
            )
            .await
            .expect("a secret basic client needs no assertion"),
        EndpointOutcome::Unavailable(ServerUnavailability::Transport(error)) if error.is_connect()
    ));
}
