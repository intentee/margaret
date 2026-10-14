use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::isolation::Isolation;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::constants::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::locked_signing_keys::locked_signing_keys;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_signing_keys::constants::SIGNING_KEY_SET_NAME;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys_tests::sealed_revision::sealed_revision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_reports_a_superseding_generation_it_cannot_observe() {
    let started = started_with_signing_keys().await;
    let backup = fresh_secret(SigningCurve::P256);
    let stored = rolled_secret(&backup);

    seeded_signing_keys(&started.database, &stored).await;

    let mut restorer = started
        .database
        .connection()
        .await
        .expect("the restoring connection is checked out");
    let restoring = restorer
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the restoring transaction begins");

    assert!(matches!(
        locked_signing_keys(&restoring, 2).await,
        Change::Changed(_)
    ));

    let synchronizer = fixture_synchronizer(started.separate_pool().await);
    let due = stored.rolled_at().after(JWKS_ROLL_INTERVAL);
    let synchronizing =
        tokio::spawn(async move { synchronizer.synchronized(&HeldSecret::Unheld, due).await });

    started.administration.await_lock_waiters(1).await;

    let backup_revision = sealed_revision(&backup);

    assert!(matches!(
        SigningKeySet::query()
            .name
            .eq(SIGNING_KEY_SET_NAME.to_string())
            .update(&restoring, |columns| {
                columns
                    .generation
                    .to(1)
                    .and(columns.nonce.to(backup_revision.sealed.nonce.to_vec()))
                    .and(
                        columns
                            .sealed_document
                            .to(backup_revision.sealed.ciphertext),
                    )
            })
            .await
            .expect("the backup is restored"),
        Change::Changed(_)
    ));
    restoring.commit().await.expect("the restore commits");

    assert!(matches!(
        synchronizing.await.expect("the synchronization joins"),
        Err(RollerError::SupersedingKeysNotObserved { expected, observed })
            if expected == SigningKeysGeneration::new(2) && observed == SigningKeysGeneration::FIRST
    ));
}
