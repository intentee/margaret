use futures_util::join;
use uuid::Uuid;

use margaret::framework::active_record::change::Change;
use margaret::framework::database::isolation::Isolation;
use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::locked_refresh_token::locked_refresh_token;
use margaret_authorization_grants_tests::opened_refresh_family::opened_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn a_sweep_waiting_on_a_rotation_of_an_expiring_family_forgets_its_rotated_token() {
    let started = started_with_authorization_grants().await;
    let expiry = contract_instant();
    let alive = NumericDate::new(expiry.seconds_since_epoch() - 1);
    let presented = contract_token();
    let next = contract_token();

    opened_refresh_family(
        &started.database,
        Uuid::new_v4(),
        RefreshFamily {
            expires_at: expiry,
            ..contract_refresh_family()
        },
        presented,
        alive,
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
    let sweeping = started.separate_pool().await;
    let (rotation, swept, ()) = join!(
        RefreshTokenRecord::rotate(&rotating, presented, next, alive),
        async {
            started.administration.await_lock_waiters(1).await;
            RefreshTokenRecord::rotate(&sweeping, contract_token(), contract_token(), expiry).await
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
    assert_eq!(
        swept.expect("the database sweeps the expired family"),
        RefreshRotation::Unknown
    );
    assert_eq!(
        RefreshTokenRecord::lookup(&started.database, next)
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
}
