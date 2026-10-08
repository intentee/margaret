use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;

#[tokio::test]
async fn synchronizer_reports_a_regressed_store() {
    let stored = fresh_secret(SigningCurve::P256);
    let held = Arc::new(rolled_secret(&stored));

    assert!(matches!(
        fixture_synchronizer(Arc::new(FixtureSigningKeys::storing(&stored)))
            .synchronized(&HeldSecret::Held(held.clone()), held.rolled_at())
            .await,
        Err(RollerError::GenerationRegressed { held, stored })
            if held == SigningKeysGeneration::new(2) && stored == SigningKeysGeneration::FIRST
    ));
}
