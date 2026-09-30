use serde_json::json;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;

#[tokio::test]
async fn reports_a_malformed_token_response() {
    assert!(matches!(
        answered_token_request(
            200,
            json!({ "token_type": "Bearer" }).to_string().into_bytes()
        )
        .await,
        EndpointOutcome::Unavailable(ServerUnavailability::MalformedAnswer { .. })
    ));
}
