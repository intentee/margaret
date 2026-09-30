use oauth2::basic::BasicErrorResponseType;
use serde_json::json;

use margaret_token_exchange_client::exchanged_token::ExchangedToken;
use margaret_token_exchange_client_tests::exchange_answered_with::exchange_answered_with;

#[tokio::test]
async fn relays_a_refused_token_exchange() {
    assert!(matches!(
        exchange_answered_with(400, &json!({ "error": "invalid_grant" })).await,
        ExchangedToken::Refused(refusal) if *refusal.error() == BasicErrorResponseType::InvalidGrant
    ));
}
