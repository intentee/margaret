use std::sync::Arc;

use futures_util::future::join_all;
use tokio::sync::Barrier;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::racing_signing_keys::RacingSigningKeys;
use margaret_registered_claims::numeric_date::NumericDate;

const INSTANCES: usize = 3;
const ROUNDS: u32 = 5;

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
async fn concurrent_roll_rounds_never_diverge() {
    let storage = Arc::new(FixtureSigningKeys::empty());
    let racing: Arc<dyn StoresSigningKeys> = Arc::new(RacingSigningKeys {
        inner: storage.clone(),
        writers: Arc::new(Barrier::new(INSTANCES)),
    });
    let mut held = vec![HeldSecret::Unheld; INSTANCES];

    for round in 0..ROUNDS {
        let now = NumericDate::new(0).after(JWKS_ROLL_INTERVAL * round);
        let synchronized: Vec<Arc<JwksSecret>> = join_all(held.iter().map(|held| {
            let synchronizer = fixture_synchronizer(racing.clone());

            async move {
                synchronizer
                    .synchronized(held, now)
                    .await
                    .expect("every instance synchronizes")
            }
        }))
        .await;

        assert!(converged(&synchronized));

        held = synchronized.into_iter().map(HeldSecret::Held).collect();
    }

    assert_eq!(
        storage.accepted_writes().await,
        usize::try_from(ROUNDS).expect("the round count fits")
    );
}
