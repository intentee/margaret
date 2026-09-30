use oauth2::basic::BasicErrorResponseType;
use serde_json::json;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client_tests::answered_grant::answered_grant;

#[tokio::test]
async fn relays_a_refused_grant() {
    assert!(matches!(
        answered_grant(400, &json!({ "error": "invalid_grant" })).await,
        EndpointOutcome::Refused(refusal) if *refusal.error() == BasicErrorResponseType::InvalidGrant
    ));
}
