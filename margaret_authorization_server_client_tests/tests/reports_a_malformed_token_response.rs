use serde_json::json;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;
use margaret_http_tests::static_handler::StaticHandler;

#[tokio::test]
async fn reports_a_malformed_token_response() {
    assert!(matches!(
        answered_token_request(StaticHandler {
            body: json!({ "token_type": "Bearer" }).to_string().into_bytes(),
            content_type: "application/json",
            status: 200,
        })
        .await,
        EndpointOutcome::Unavailable(ServerUnavailability::MalformedAnswer { .. })
    ));
}
