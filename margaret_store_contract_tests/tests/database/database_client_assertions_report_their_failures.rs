use std::sync::Arc;

use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_client_assertions_database::client_assertions_database_error::ClientAssertionsDatabaseError;
use margaret_client_assertions_database::database_client_assertions::DatabaseClientAssertions;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_store_contract_tests::contract_assertion_expiry::contract_assertion_expiry;
use margaret_store_contract_tests::contract_instant::contract_instant;
use margaret_store_contract_tests::contract_token::contract_token;

#[tokio::test]
async fn reports_a_remembrance_in_a_database_without_its_table() {
    let started = StartedDatabase::start().await;
    let error = DatabaseClientAssertions::create(Arc::clone(&started.database))
        .remember_client_assertion(
            "contract-client",
            contract_token(),
            contract_assertion_expiry(),
            contract_instant(),
        )
        .await
        .expect_err("the remembrance fails without the client assertions table");

    assert!(matches!(
        error.downcast_ref::<ClientAssertionsDatabaseError>(),
        Some(ClientAssertionsDatabaseError::Remember(_))
    ));
}
