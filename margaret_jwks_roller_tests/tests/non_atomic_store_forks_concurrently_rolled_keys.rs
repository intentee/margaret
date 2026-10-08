use std::sync::Arc;

use futures_util::future::join_all;
use tokio::sync::Barrier;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::non_atomic_signing_keys::NonAtomicSigningKeys;
use margaret_jwks_roller_tests::racing_signing_keys::RacingSigningKeys;

const INSTANCES: usize = 2;

#[tokio::test]
async fn non_atomic_store_forks_concurrently_rolled_keys() {
    let held = Arc::new(fresh_secret(SigningCurve::P256));
    let racing: Arc<dyn StoresSigningKeys> = Arc::new(RacingSigningKeys {
        inner: Arc::new(NonAtomicSigningKeys::new(StoredSigningKeys::Stored(
            SigningKeysRevision::from_secret(&held).expect("the held keys serialize"),
        ))),
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
                .expect("every instance believes it rolled the keys")
        }
    }))
    .await;
    let outcomes = join_all(rolled.into_iter().map(|secret| {
        let synchronizer = fixture_synchronizer(racing.clone());

        async move {
            synchronizer
                .synchronized(&HeldSecret::Held(secret), due)
                .await
        }
    }))
    .await;

    assert!(
        outcomes
            .iter()
            .any(|outcome| matches!(outcome, Err(RollerError::GenerationForked { .. })))
    );
}
