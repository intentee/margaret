use uuid::Uuid;

use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_refresh_family_revoked_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        RefreshFamilyRecord::revoke(&started.database, Uuid::new_v4()).await,
        Err(AuthorizationGrantsError::RevokeRefreshFamily(_))
    ));
}
