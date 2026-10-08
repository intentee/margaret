use margaret_client_assertions::client_assertion::ClientAssertion;
use margaret_client_assertions::client_assertions_error::ClientAssertionsError;
use margaret_client_assertions_tests::contract_assertion_expiry::contract_assertion_expiry;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_remembrance_in_a_database_without_its_table() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        ClientAssertion::remember(
            &started.database,
            "contract-client",
            contract_token(),
            contract_assertion_expiry(),
            contract_instant(),
        )
        .await,
        Err(ClientAssertionsError::Sweep(_))
    ));
}
