use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;
use margaret_http_tests::static_handler::StaticHandler;

#[tokio::test]
async fn reports_an_empty_successful_token_answer_as_unavailable_since_it_carries_no_token() {
    assert!(matches!(
        answered_token_request(StaticHandler {
            body: Vec::new(),
            content_type: "application/json",
            status: 200,
        })
        .await,
        EndpointOutcome::Unavailable(ServerUnavailability::EmptyAnswer)
    ));
}
