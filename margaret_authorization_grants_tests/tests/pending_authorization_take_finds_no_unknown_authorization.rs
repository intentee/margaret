use uuid::Uuid;

use margaret_authorization_grants::pending_authorization_record::PendingAuthorizationRecord;
use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;

#[tokio::test]
async fn pending_authorization_take_finds_no_unknown_authorization() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();

    assert_eq!(
        PendingAuthorizationRecord::take(database, Uuid::new_v4())
            .await
            .expect("the database takes pending authorizations"),
        PendingAuthorizationTake::Absent
    );
}
