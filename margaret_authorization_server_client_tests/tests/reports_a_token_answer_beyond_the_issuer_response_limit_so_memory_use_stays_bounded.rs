use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_issuer_request::issuer_response_max_bytes::ISSUER_RESPONSE_MAX_BYTES;

#[tokio::test]
async fn reports_a_token_answer_beyond_the_issuer_response_limit_so_memory_use_stays_bounded() {
    assert!(matches!(
        answered_token_request(StaticHandler {
            body: vec![b' '; ISSUER_RESPONSE_MAX_BYTES + 1],
            content_type: "application/json",
            status: 200,
        })
        .await,
        EndpointOutcome::Unavailable(ServerUnavailability::OversizedAnswer {
            max_bytes: ISSUER_RESPONSE_MAX_BYTES
        })
    ));
}
