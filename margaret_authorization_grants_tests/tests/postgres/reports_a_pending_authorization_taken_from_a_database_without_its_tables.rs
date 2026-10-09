use uuid::Uuid;

use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::pending_authorization_record::PendingAuthorizationRecord;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_pending_authorization_taken_from_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        PendingAuthorizationRecord::take(&started.database, Uuid::new_v4()).await,
        Err(AuthorizationGrantsError::TakePendingAuthorization(_))
    ));
}
