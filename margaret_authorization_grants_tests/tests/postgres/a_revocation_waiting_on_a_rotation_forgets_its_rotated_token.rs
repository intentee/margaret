use futures_util::join;
use uuid::Uuid;

use margaret::framework::active_record::change::Change;
use margaret::framework::database::isolation::Isolation;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::locked_refresh_token::locked_refresh_token;
use margaret_authorization_grants_tests::opened_refresh_family::opened_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn a_revocation_waiting_on_a_rotation_forgets_its_rotated_token() {
    let started = started_with_authorization_grants().await;
    let family = Uuid::new_v4();
    let presented = contract_token();
    let next = contract_token();
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
    let holding = gate
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the gate transaction begins");

    assert!(matches!(
        locked_refresh_token(&holding, presented).await,
        Change::Changed(_)
    ));

    let rotating = started.separate_pool().await;
    let revoking = started.separate_pool().await;
    let (rotation, revocation, ()) = join!(
        RefreshTokenRecord::rotate(&rotating, presented, next, now),
        async {
            started.administration.await_lock_waiters(1).await;
            RefreshFamilyRecord::revoke(&revoking, family).await
        },
        async {
            started.administration.await_lock_waiters(2).await;
            holding.rollback().await.expect("the gate opens");
        }
    );

    assert_eq!(
        rotation.expect("the database rotates the refresh token"),
        RefreshRotation::Rotated
    );
    revocation.expect("the database revokes the refresh family");
    assert_eq!(
        RefreshTokenRecord::lookup(&started.database, next)
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
}
