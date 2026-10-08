use futures_util::future::join_all;
use uuid::Uuid;

use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::pending_authorization_record::PendingAuthorizationRecord;
use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_authorization_grants_tests::contract_pending_authorization::contract_pending_authorization;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::racing_instances::RACING_INSTANCES;

#[tokio::test]
async fn concurrent_pending_authorization_takes_admit_one() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let id = Uuid::new_v4();
    let pending = contract_pending_authorization();

    PendingAuthorizationRecord::hold(database, id, pending.clone(), contract_instant())
        .await
        .expect("the database holds the pending authorization");

    let taken: Vec<PendingAuthorization> = join_all((0..RACING_INSTANCES).map(|_| async move {
        match PendingAuthorizationRecord::take(database, id)
            .await
            .expect("the database takes the pending authorization")
        {
            PendingAuthorizationTake::Absent => None,
            PendingAuthorizationTake::Taken(taken) => Some(*taken),
        }
    }))
    .await
    .into_iter()
    .flatten()
    .collect();

    assert_eq!(taken, [pending]);
}
