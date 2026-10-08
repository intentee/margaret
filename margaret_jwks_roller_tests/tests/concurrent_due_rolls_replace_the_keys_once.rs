use std::sync::Arc;

use futures_util::future::join_all;
use tokio::sync::Barrier;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::racing_signing_keys::RacingSigningKeys;

const INSTANCES: usize = 3;

fn converged(secrets: &[Arc<JwksSecret>]) -> bool {
    secrets.windows(2).all(|pair| {
        pair[0].generation() == pair[1].generation()
            && pair[0].current().kid() == pair[1].current().kid()
            && pair[0].next().kid() == pair[1].next().kid()
            && serde_json::to_vec(pair[0].public_jwks()).expect("the published set serializes")
                == serde_json::to_vec(pair[1].public_jwks()).expect("the published set serializes")
    })
}

#[tokio::test]
async fn concurrent_due_rolls_replace_the_keys_once() {
    let held = Arc::new(fresh_secret(SigningCurve::P256));
    let storage = Arc::new(FixtureSigningKeys::storing(&held));
    let racing: Arc<dyn StoresSigningKeys> = Arc::new(RacingSigningKeys {
        inner: storage.clone(),
        writers: Arc::new(Barrier::new(INSTANCES)),
    });
    let due = held.rolled_at().after(JWKS_ROLL_INTERVAL);
    let rolled: Vec<Arc<JwksSecret>> = join_all((0..INSTANCES).map(|_| {
        let synchronizer = fixture_synchronizer(racing.clone());
        let held = HeldSecret::Held(held.clone());

        async move {
            synchronizer
                .synchronized(&held, due)
                .await
                .expect("every instance rolls")
        }
    }))
    .await;

    assert!(converged(&rolled));
    assert_eq!(rolled[0].generation(), SigningKeysGeneration::new(2));
    assert_eq!(rolled[0].current().kid(), held.next().kid());
    assert_eq!(storage.accepted_writes().await, 1);
}
