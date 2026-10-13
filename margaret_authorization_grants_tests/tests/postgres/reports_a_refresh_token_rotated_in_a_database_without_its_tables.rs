use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_refresh_token_rotated_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        RefreshTokenRecord::rotate(
            &started.database,
            contract_token(),
            contract_token(),
            contract_instant(),
        )
        .await,
        Err(AuthorizationGrantsError::SweepRefreshFamilies(_))
    ));
}
