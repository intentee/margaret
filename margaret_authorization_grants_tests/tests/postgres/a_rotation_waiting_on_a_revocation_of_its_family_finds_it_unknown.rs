use futures_util::join;
use uuid::Uuid;

use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::removal::Removal;
use margaret::framework::database::isolation::Isolation;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::opened_refresh_family::opened_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn a_rotation_waiting_on_a_revocation_of_its_family_finds_it_unknown() {
    let started = started_with_authorization_grants().await;
    let family = Uuid::new_v4();
    let presented = contract_token();
    let now = contract_instant();

    opened_refresh_family(
        &started.database,
        family,
        contract_refresh_family(),
        presented,
        now,
    )
    .await;

    let mut gate = started
        .database
        .connection()
        .await
        .expect("the gate connection is checked out");
    let revoking = gate
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the revoking transaction begins");

    assert!(matches!(
        RefreshFamilyRecord::query()
            .id
            .eq(family)
            .delete(&revoking)
            .await
            .expect("the revoking transaction forgets the family"),
        Removal::Removed(_)
    ));

    let rotating = started.separate_pool().await;
    let (rotation, ()) = join!(
        RefreshTokenRecord::rotate(&rotating, presented, contract_token(), now),
        async {
            started.administration.await_lock_waiters(1).await;
            revoking.commit().await.expect("the revocation commits");
        }
    );

    assert_eq!(
        rotation.expect("the database rotates refresh tokens"),
        RefreshRotation::Unknown
    );
}
