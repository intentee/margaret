use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_reports_a_regressed_store() {
    let started = started_with_signing_keys().await;
    let stored = fresh_secret(SigningCurve::P256);
    let held = Arc::new(rolled_secret(&stored));

    seeded_signing_keys(&started.database, &stored).await;

    assert!(matches!(
        fixture_synchronizer(Arc::clone(&started.database))
            .synchronized(&HeldSecret::Held(held.clone()), held.rolled_at())
            .await,
        Err(RollerError::GenerationRegressed { held, stored })
            if held == SigningKeysGeneration::new(2) && stored == SigningKeysGeneration::FIRST
    ));
}
