use uuid::Uuid;

use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_refresh_family_opened_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        RefreshFamilyRecord::open(
            &started.database,
            Uuid::new_v4(),
            contract_refresh_family(),
            contract_token(),
            contract_instant(),
        )
        .await,
        Err(AuthorizationGrantsError::SweepRefreshFamilies(_))
    ));
}
