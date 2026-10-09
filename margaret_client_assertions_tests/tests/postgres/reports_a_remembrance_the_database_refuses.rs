use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_client_assertions::client_assertion::ClientAssertion;
use margaret_client_assertions::client_assertions_error::ClientAssertionsError;
use margaret_client_assertions_tests::contract_assertion_expiry::contract_assertion_expiry;
use margaret_client_assertions_tests::started_with_client_assertions::started_with_client_assertions;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::table_privilege::TablePrivilege;

#[tokio::test]
async fn reports_a_remembrance_the_database_refuses() {
    let started = started_with_client_assertions().await;

    started
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "client_assertions",
        )
        .await;

    assert!(matches!(
        ClientAssertion::remember(
            &started.database,
            "contract-client",
            contract_token(),
            contract_assertion_expiry(),
            contract_instant(),
        )
        .await,
        Err(ClientAssertionsError::Remember(_))
    ));
}
