use std::sync::Arc;

use futures_util::future::join_all;

use margaret::framework::database::isolation::Isolation;
use margaret_database_tests::racing_instances::RACING_INSTANCES;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller_tests::converged::converged;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::held_signing_keys_creation::held_signing_keys_creation;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn concurrent_starts_on_an_empty_store_create_one_secret() {
    let started = started_with_signing_keys().await;
    let mut gate = started
        .database
        .connection()
        .await
        .expect("the gate connection is checked out");
    let holding = gate
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the gate transaction begins");

    held_signing_keys_creation(&holding, &fresh_secret(SigningCurve::P256)).await;

    let mut instances = Vec::new();

    for _ in 0..RACING_INSTANCES {
        let synchronizer = fixture_synchronizer(started.separate_pool().await);

        instances.push(tokio::spawn(async move {
            synchronizer
                .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
                .await
                .expect("every instance starts")
        }));
    }

    started
        .administration
        .await_lock_waiters(i64::try_from(RACING_INSTANCES).expect("the instances fit"))
        .await;
    holding.rollback().await.expect("the gate opens");

    let started_secrets: Vec<Arc<JwksSecret>> = join_all(instances)
        .await
        .into_iter()
        .map(|joined| joined.expect("the instance joins"))
        .collect();

    assert!(converged(&started_secrets));
    assert_eq!(
        StoredRevision::loaded(&started.database).await,
        StoredRevision::of(
            &SigningKeysRevision::from_secret(&started_secrets[0]).expect("the keys serialize")
        )
    );
}
