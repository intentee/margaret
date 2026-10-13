use serde_json::json;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_token_exchange_client::exchanged_token::ExchangedToken;
use margaret_token_exchange_client_tests::exchange_answered_with::exchange_answered_with;

#[tokio::test]
async fn reports_a_502_from_the_token_endpoint_as_unavailable_rather_than_a_refusal() {
    assert!(matches!(
        exchange_answered_with(502, &json!({})).await,
        ExchangedToken::Unavailable(ServerUnavailability::MalformedAnswer { .. })
    ));
}
