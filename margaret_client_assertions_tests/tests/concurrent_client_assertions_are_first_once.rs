use futures_util::future::join_all;

use margaret_client_assertions::assertion_memory::AssertionMemory;
use margaret_client_assertions::client_assertion::ClientAssertion;
use margaret_client_assertions_tests::contract_assertion_expiry::contract_assertion_expiry;
use margaret_client_assertions_tests::started_with_client_assertions::started_with_client_assertions;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::racing_instances::RACING_INSTANCES;

#[tokio::test]
async fn concurrent_client_assertions_are_first_once() {
    let started = started_with_client_assertions().await;
    let database = started.database.as_ref();
    let assertion = contract_token();
    let expires_at = contract_assertion_expiry();
    let now = contract_instant();
    let memories: Vec<AssertionMemory> = join_all((0..RACING_INSTANCES).map(|_| async move {
        ClientAssertion::remember(database, "contract-client", assertion, expires_at, now)
            .await
            .expect("the client assertion is remembered")
    }))
    .await;

    assert_eq!(
        memories
            .iter()
            .filter(|memory| **memory == AssertionMemory::First)
            .count(),
        1
    );
}
