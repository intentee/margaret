use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;
use margaret_issuer_request::issuer_response_max_bytes::ISSUER_RESPONSE_MAX_BYTES;

#[tokio::test]
async fn reports_an_oversized_answer() {
    assert!(matches!(
        answered_token_request(200, vec![b' '; ISSUER_RESPONSE_MAX_BYTES + 1]).await,
        EndpointOutcome::Unavailable(ServerUnavailability::Oversized {
            max_bytes: ISSUER_RESPONSE_MAX_BYTES
        })
    ));
}
