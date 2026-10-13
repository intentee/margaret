use std::sync::Arc;

use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn synchronizer_creates_fresh_keys_in_an_empty_store() {
    let started = started_with_signing_keys().await;
    let now = NumericDate::new(0);
    let created = fixture_synchronizer(Arc::clone(&started.database))
        .synchronized(&HeldSecret::Unheld, now)
        .await
        .expect("fresh keys are created");

    assert_eq!(created.generation(), SigningKeysGeneration::FIRST);
    assert_eq!(created.rolled_at(), now);
    assert!(created.retired().is_empty());
    assert_eq!(
        StoredRevision::loaded(&started.database).await,
        StoredRevision::of(
            &SigningKeysRevision::from_secret(&created).expect("the keys serialize")
        )
    );
}
