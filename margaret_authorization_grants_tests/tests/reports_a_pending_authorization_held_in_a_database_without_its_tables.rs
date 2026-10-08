use uuid::Uuid;

use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_authorization_grants::pending_authorization_record::PendingAuthorizationRecord;
use margaret_authorization_grants_tests::contract_pending_authorization::contract_pending_authorization;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn reports_a_pending_authorization_held_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        PendingAuthorizationRecord::hold(
            &started.database,
            Uuid::new_v4(),
            contract_pending_authorization(),
            contract_instant(),
        )
        .await,
        Err(AuthorizationGrantsError::SweepPendingAuthorizations(_))
    ));
}
