use std::sync::Arc;

use futures_util::future::join_all;

use margaret::framework::active_record::change::Change;
use margaret::framework::database::isolation::Isolation;
use margaret_database_tests::racing_instances::RACING_INSTANCES;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller_tests::converged::converged;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::locked_signing_keys::locked_signing_keys;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn concurrent_due_rolls_replace_the_keys_once() {
    let started = started_with_signing_keys().await;
    let held = Arc::new(fresh_secret(SigningCurve::P256));

    seeded_signing_keys(&started.database, &held).await;

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
        locked_signing_keys(&holding, 1).await,
        Change::Changed(_)
    ));

    let due = held.rolled_at().after(JWKS_ROLL_INTERVAL);
    let mut instances = Vec::new();

    for _ in 0..RACING_INSTANCES {
        let synchronizer = fixture_synchronizer(started.separate_pool().await);
        let held = HeldSecret::Held(held.clone());

        instances.push(tokio::spawn(async move {
            synchronizer
                .synchronized(&held, due)
                .await
                .expect("every instance rolls")
        }));
    }

    started
        .administration
        .await_lock_waiters(i64::try_from(RACING_INSTANCES).expect("the instances fit"))
        .await;
    holding.rollback().await.expect("the gate opens");

    let rolled: Vec<Arc<JwksSecret>> = join_all(instances)
        .await
        .into_iter()
        .map(|joined| joined.expect("the instance joins"))
        .collect();

    assert!(converged(&rolled));
    assert_eq!(rolled[0].generation(), SigningKeysGeneration::new(2));
    assert_eq!(rolled[0].current().kid(), held.next().kid());
    assert_eq!(
        StoredRevision::loaded(&started.database).await,
        StoredRevision::of(
            &SigningKeysRevision::from_secret(&rolled[0]).expect("the keys serialize")
        )
    );
}
