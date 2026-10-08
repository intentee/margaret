use margaret_client_assertions::assertion_memory::AssertionMemory;
use margaret_client_assertions::client_assertion::ClientAssertion;
use margaret_client_assertions_tests::contract_assertion_expiry::contract_assertion_expiry;
use margaret_client_assertions_tests::started_with_client_assertions::started_with_client_assertions;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn client_assertions_are_remembered_per_client() {
    let started = started_with_client_assertions().await;
    let assertion = contract_token();
    let expires_at = contract_assertion_expiry();
    let now = contract_instant();

    for client_id in ["contract-client", "another-contract-client"] {
        assert_eq!(
            ClientAssertion::remember(&started.database, client_id, assertion, expires_at, now)
                .await
                .expect("the client assertion is remembered"),
            AssertionMemory::First
        );
    }
}
