use uuid::Uuid;

use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::pending_authorization_record::PendingAuthorizationRecord;
use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_authorization_grants_tests::contract_pending_authorization::contract_pending_authorization;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;

#[tokio::test]
async fn holding_a_pending_authorization_sweeps_expired_ones() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let now = contract_instant();
    let expired = Uuid::new_v4();

    PendingAuthorizationRecord::hold(
        database,
        expired,
        PendingAuthorization {
            expires_at: now,
            ..contract_pending_authorization()
        },
        now,
    )
    .await
    .expect("the database holds the expiring pending authorization");
    PendingAuthorizationRecord::hold(
        database,
        Uuid::new_v4(),
        contract_pending_authorization(),
        now,
    )
    .await
    .expect("the database holds another pending authorization");

    assert_eq!(
        PendingAuthorizationRecord::take(database, expired)
            .await
            .expect("the database takes pending authorizations"),
        PendingAuthorizationTake::Absent
    );
}
