use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;

#[tokio::test]
async fn reports_an_empty_refusal() {
    assert!(matches!(
        answered_token_request(503, Vec::new()).await,
        EndpointOutcome::Unavailable(ServerUnavailability::UnexpectedAnswer { .. })
    ));
}
