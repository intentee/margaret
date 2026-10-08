use std::sync::Arc;

use futures_util::future::join_all;

use margaret::framework::active_record::change::Change;
use margaret::framework::database::isolation::Isolation;
use margaret_database::database::Database;
use margaret_database_tests::racing_instances::RACING_INSTANCES;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller_tests::converged::converged;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::held_signing_keys_creation::held_signing_keys_creation;
use margaret_jwks_roller_tests::locked_signing_keys::locked_signing_keys;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

const ROUNDS: u32 = 5;

async fn raced_round(
    started: &StartedDatabase,
    pools: &[Arc<Database>],
    held: &[HeldSecret],
    now: NumericDate,
) -> Vec<Arc<JwksSecret>> {
    let mut gate = started
        .database
        .connection()
        .await
        .expect("the gate connection is checked out");
    let holding = gate
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the gate transaction begins");

    match &held[0] {
        HeldSecret::Unheld => {
            held_signing_keys_creation(&holding, &fresh_secret(SigningCurve::P256)).await;
        }
        HeldSecret::Held(secret) => {
            assert!(matches!(
                locked_signing_keys(
                    &holding,
                    i64::try_from(secret.generation().value()).expect("the generation fits"),
                )
                .await,
                Change::Changed(_)
            ));
        }
    }

    let instances: Vec<_> = pools
        .iter()
        .zip(held)
        .map(|(pool, held)| {
            let synchronizer = fixture_synchronizer(Arc::clone(pool));
            let held = held.clone();

            tokio::spawn(async move {
                synchronizer
                    .synchronized(&held, now)
                    .await
                    .expect("every instance synchronizes")
            })
        })
        .collect();

    started
        .administration
        .await_lock_waiters(i64::try_from(RACING_INSTANCES).expect("the instances fit"))
        .await;
    holding.rollback().await.expect("the gate opens");

    join_all(instances)
        .await
        .into_iter()
        .map(|joined| joined.expect("the instance joins"))
        .collect()
}

#[tokio::test]
async fn concurrent_roll_rounds_never_diverge() {
    let started = started_with_signing_keys().await;
    let pools: Vec<Arc<Database>> =
        join_all((0..RACING_INSTANCES).map(|_| started.separate_pool())).await;
    let mut held = vec![HeldSecret::Unheld; RACING_INSTANCES];

    for round in 0..ROUNDS {
        let synchronized = raced_round(
            &started,
            &pools,
            &held,
            NumericDate::new(0).after(JWKS_ROLL_INTERVAL * round),
        )
        .await;

        assert!(converged(&synchronized));

        held = synchronized.into_iter().map(HeldSecret::Held).collect();
    }

    assert!(matches!(
        StoredRevision::loaded(&started.database).await,
        StoredRevision::Stored { generation, .. } if generation == SigningKeysGeneration::new(u64::from(ROUNDS))
    ));
}
