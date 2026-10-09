use margaret::framework::active_record::change::Change;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::isolation::Isolation;
use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::locked_signing_keys::locked_signing_keys;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_key_set_name::SIGNING_KEY_SET_NAME;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_reports_an_outage_after_a_superseded_roll() {
    let started = started_with_signing_keys().await;
    let stored = fresh_secret(SigningCurve::P256);

    seeded_signing_keys(&started.database, &stored).await;

    let mut peer = started
        .database
        .connection()
        .await
        .expect("the peer connection is checked out");
    let rolling = peer
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the peer transaction begins");

    assert!(matches!(
        locked_signing_keys(&rolling, 1).await,
        Change::Changed(_)
    ));

    let synchronizer = fixture_synchronizer(started.separate_pool().await);
    let due = stored.rolled_at().after(JWKS_ROLL_INTERVAL);
    let synchronizing =
        tokio::spawn(async move { synchronizer.synchronized(&HeldSecret::Unheld, due).await });

    started.administration.await_lock_waiters(1).await;

    let peer_roll = SigningKeysRevision::from_secret(&rolled_secret(&stored))
        .expect("the peer roll serializes");

    assert!(matches!(
        SigningKeySet::query()
            .name
            .eq(SIGNING_KEY_SET_NAME.to_string())
            .update(&rolling, |columns| {
                columns
                    .generation
                    .to(2)
                    .and(columns.document.to(peer_roll.document))
            })
            .await
            .expect("the peer rolls the keys"),
        Change::Changed(_)
    ));
    started
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Framework,
            "signing_key_sets",
        )
        .await;
    rolling.commit().await.expect("the peer roll commits");

    assert!(matches!(
        synchronizing.await.expect("the synchronization joins"),
        Err(RollerError::SecretLoad { .. })
    ));
}
