use std::sync::Arc;

use futures_util::future::join_all;
use tokio::sync::Barrier;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::non_atomic_signing_keys::NonAtomicSigningKeys;
use margaret_jwks_roller_tests::racing_signing_keys::RacingSigningKeys;
use margaret_registered_claims::numeric_date::NumericDate;

const INSTANCES: usize = 2;

#[tokio::test]
async fn non_atomic_store_forks_concurrently_created_keys() {
    let racing: Arc<dyn StoresSigningKeys> = Arc::new(RacingSigningKeys {
        inner: Arc::new(NonAtomicSigningKeys::new(StoredSigningKeys::Absent)),
        writers: Arc::new(Barrier::new(INSTANCES)),
    });
    let started: Vec<Arc<JwksSecret>> = join_all((0..INSTANCES).map(|_| {
        let synchronizer = fixture_synchronizer(racing.clone());

        async move {
            synchronizer
                .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
                .await
                .expect("every instance believes it created the keys")
        }
    }))
    .await;
    let forked = started.into_iter().map(|secret| {
        let synchronizer = fixture_synchronizer(racing.clone());

        async move {
            synchronizer
                .synchronized(&HeldSecret::Held(secret), NumericDate::new(0))
                .await
        }
    });
    let outcomes = join_all(forked).await;

    assert!(
        outcomes
            .iter()
            .any(|outcome| matches!(outcome, Err(RollerError::GenerationForked { .. })))
    );
}
