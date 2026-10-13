use serde_json::json;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_grant::answered_grant;

#[tokio::test]
async fn reports_a_tokenless_grant_answer_as_malformed_rather_than_as_a_success() {
    assert!(matches!(
        answered_grant(200, &json!({ "token_type": "Bearer" })).await,
        EndpointOutcome::Unavailable(ServerUnavailability::MalformedAnswer { .. })
    ));
}
