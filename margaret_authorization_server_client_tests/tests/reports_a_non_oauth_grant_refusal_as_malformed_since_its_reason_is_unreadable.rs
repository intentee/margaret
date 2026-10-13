use serde_json::json;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_grant::answered_grant;

#[tokio::test]
async fn reports_a_non_oauth_grant_refusal_as_malformed_since_its_reason_is_unreadable() {
    assert!(matches!(
        answered_grant(401, &json!({ "message": "unauthorized" })).await,
        EndpointOutcome::Unavailable(ServerUnavailability::MalformedAnswer { .. })
    ));
}
