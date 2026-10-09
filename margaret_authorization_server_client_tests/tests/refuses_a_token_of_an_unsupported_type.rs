use oauth2::basic::BasicTokenType;
use serde_json::json;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;
use margaret_http_tests::static_handler::StaticHandler;

#[tokio::test]
async fn refuses_a_token_of_an_unsupported_type() {
    assert!(matches!(
           answered_token_request(StaticHandler {
    body: json!({ "access_token": "token", "token_type": "DPoP" })
                   .to_string()
                   .into_bytes(),
    content_type: "application/json",
    status: 200,
    })
           .await,
           EndpointOutcome::Unavailable(ServerUnavailability::UnsupportedTokenType { token_type })
               if token_type == BasicTokenType::Extension("dpop".to_string())
       ));
}
