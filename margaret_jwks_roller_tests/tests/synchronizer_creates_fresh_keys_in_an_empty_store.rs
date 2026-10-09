use std::sync::Arc;

use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys as _;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn synchronizer_creates_fresh_keys_in_an_empty_store() {
    let storage = Arc::new(FixtureSigningKeys::empty());
    let now = NumericDate::new(0);
    let created = fixture_synchronizer(storage.clone())
        .synchronized(&HeldSecret::Unheld, now)
        .await
        .expect("fresh keys are created");
    let StoredSigningKeys::Stored(revision) = storage
        .load_signing_keys()
        .await
        .expect("the store is reachable")
    else {
        panic!("the created keys are stored");
    };

    assert_eq!(created.generation(), SigningKeysGeneration::FIRST);
    assert_eq!(created.rolled_at(), now);
    assert!(created.retired().is_empty());
    assert_eq!(revision.generation, SigningKeysGeneration::FIRST);
    assert_eq!(storage.accepted_writes().await, 1);
}
