use uuid::Uuid;

use margaret::framework::active_record::change::Change;
use margaret::framework::database::isolation::Isolation;
use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_family_revocation_record::RefreshFamilyRevocationRecord;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::locked_refresh_token::locked_refresh_token;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn a_rotation_waiting_on_its_token_honors_a_revocation_committed_meanwhile() {
    let started = started_with_authorization_grants().await;
    let family = Uuid::new_v4();
    let presented = contract_token();
    let now = contract_instant();

    assert_eq!(
        RefreshFamilyRecord::open(
            &started.database,
            family,
            contract_refresh_family(),
            presented,
            now
        )
        .await
        .expect("the database opens the refresh family"),
        FamilyOpening::Opened
    );

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
    let rotation = tokio::spawn(async move {
        RefreshTokenRecord::rotate(&rotating, presented, contract_token(), now)
            .await
            .expect("the database rotates refresh tokens")
    });

    started.administration.await_lock_waiters(1).await;
    RefreshFamilyRevocationRecord::revoke(&started.database, family, now)
        .await
        .expect("the database revokes the refresh family");
    holding.rollback().await.expect("the gate opens");

    assert_eq!(
        rotation.await.expect("the rotation completes"),
        RefreshRotation::Unknown
    );
}
