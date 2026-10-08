use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants_tests::contract_issued_code::contract_issued_code;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_code_issued_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        AuthorizationCodeRecord::issue(
            &started.database,
            contract_token(),
            contract_issued_code(),
            contract_instant(),
        )
        .await,
        Err(AuthorizationGrantsError::SweepAuthorizationCodes(_))
    ));
}
