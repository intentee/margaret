use margaret_client_assertions::assertion_memory::AssertionMemory;
use margaret_client_assertions::client_assertion::ClientAssertion;
use margaret_client_assertions_tests::contract_assertion_expiry::contract_assertion_expiry;
use margaret_client_assertions_tests::started_with_client_assertions::started_with_client_assertions;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn client_assertions_are_first_again_once_expired() {
    let started = started_with_client_assertions().await;
    let database = started.database.as_ref();
    let assertion = contract_token();
    let expires_at = contract_assertion_expiry();
    let remembered_at = |now: NumericDate| async move {
        ClientAssertion::remember(database, "contract-client", assertion, expires_at, now)
            .await
            .expect("the client assertion is remembered")
    };

    assert_eq!(
        remembered_at(contract_instant()).await,
        AssertionMemory::First
    );
    assert_eq!(
        remembered_at(NumericDate::new(expires_at.seconds_since_epoch() - 1)).await,
        AssertionMemory::Seen
    );
    assert_eq!(remembered_at(expires_at).await, AssertionMemory::First);
}
