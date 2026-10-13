use http::StatusCode;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;
use margaret_http_tests::static_handler::StaticHandler;

#[tokio::test]
async fn reports_an_empty_refusal_as_unavailable_with_its_status_since_no_oauth_error_is_given() {
    assert!(matches!(
        answered_token_request(StaticHandler {
            body: Vec::new(),
            content_type: "application/json",
            status: 503,
        })
        .await,
        EndpointOutcome::Unavailable(ServerUnavailability::EmptyRefusal {
            status: StatusCode::SERVICE_UNAVAILABLE
        },)
    ));
}
