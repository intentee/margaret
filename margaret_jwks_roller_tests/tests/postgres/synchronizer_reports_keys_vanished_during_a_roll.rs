use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::removal::Removal;
use margaret::framework::database::isolation::Isolation;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::locked_signing_keys::locked_signing_keys;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_key_set_name::SIGNING_KEY_SET_NAME;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_reports_keys_vanished_during_a_roll() {
    let started = started_with_signing_keys().await;
    let stored = fresh_secret(SigningCurve::P256);

    seeded_signing_keys(&started.database, &stored).await;

    let mut remover = started
        .database
        .connection()
        .await
        .expect("the removing connection is checked out");
    let removing = remover
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the removing transaction begins");

    assert!(matches!(
        locked_signing_keys(&removing, 1).await,
        Change::Changed(_)
    ));

    let synchronizer = fixture_synchronizer(started.separate_pool().await);
    let due = stored.rolled_at().after(JWKS_ROLL_INTERVAL);
    let synchronizing =
        tokio::spawn(async move { synchronizer.synchronized(&HeldSecret::Unheld, due).await });

    started.administration.await_lock_waiters(1).await;

    assert!(matches!(
        SigningKeySet::query()
            .name
            .eq(SIGNING_KEY_SET_NAME.to_string())
            .delete(&removing)
            .await
            .expect("the stored keys are removed"),
        Removal::Removed(_)
    ));
    removing.commit().await.expect("the removal commits");

    assert!(matches!(
        synchronizing.await.expect("the synchronization joins"),
        Err(RollerError::StoredKeysVanished { generation }) if generation == SigningKeysGeneration::FIRST
    ));
}
