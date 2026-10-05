use std::sync::Arc;

use oauth2::EmptyExtraTokenFields;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn reports_an_unreachable_grant_endpoint() {
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
            .request_grant::<EmptyExtraTokenFields>(
                GrantType::TokenExchange,
                Vec::new(),
            )
            .await,
        EndpointOutcome::Unavailable(ServerUnavailability::Exchange(IssuerExchangeError::Transport(error))) if error.is_connect()
    ));
}
