use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;

#[tokio::test]
async fn synchronizer_reports_keys_vanished_from_the_store() {
    let held = Arc::new(fresh_secret(SigningCurve::P256));
    let storage = Arc::new(FixtureSigningKeys::storing(&held));

    storage.overwrite(StoredSigningKeys::Absent).await;

    assert!(matches!(
        fixture_synchronizer(storage)
            .synchronized(&HeldSecret::Held(held.clone()), held.rolled_at())
            .await,
        Err(RollerError::StoredKeysVanished { generation }) if generation == SigningKeysGeneration::FIRST
    ));
}
