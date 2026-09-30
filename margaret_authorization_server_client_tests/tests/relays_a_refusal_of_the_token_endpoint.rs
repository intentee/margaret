use oauth2::basic::BasicErrorResponseType;
use serde_json::json;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;

#[tokio::test]
async fn relays_a_refusal_of_the_token_endpoint() {
    let EndpointOutcome::Refused(refusal) = answered_token_request(
        400,
        json!({ "error": "invalid_scope", "error_description": "the scope is unknown" })
            .to_string()
            .into_bytes(),
    )
    .await
    else {
        panic!("the token request is refused");
    };

    assert_eq!(refusal.error(), &BasicErrorResponseType::InvalidScope);
    assert_eq!(
        refusal.error_description().map(String::as_str),
        Some("the scope is unknown")
    );
}
