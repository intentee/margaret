use serde_json::json;

use margaret_token_exchange_client::exchanged_token::ExchangedToken;
use margaret_token_exchange_client::issued_token_type_mismatch::IssuedTokenTypeMismatch;
use margaret_token_exchange_client_tests::exchange_answered_with::exchange_answered_with;

#[tokio::test]
async fn reports_an_exchange_without_an_issued_token_type() {
    assert!(matches!(
        exchange_answered_with(
            200,
            &json!({ "access_token": "token", "token_type": "Bearer" })
        )
        .await,
        ExchangedToken::IssuedTokenTypeMismatch(IssuedTokenTypeMismatch::Missing)
    ));
}
