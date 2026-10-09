use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;

#[tokio::test]
async fn synchronizer_adopts_a_newer_generation_without_writing() {
    let held = Arc::new(fresh_secret(SigningCurve::P256));
    let newer = rolled_secret(&rolled_secret(&rolled_secret(&held)));
    let storage = Arc::new(FixtureSigningKeys::storing(&newer));
    let adopted = fixture_synchronizer(storage.clone())
        .synchronized(&HeldSecret::Held(held), newer.rolled_at())
        .await
        .expect("the newer keys are adopted");

    assert_eq!(adopted.generation(), newer.generation());
    assert_eq!(adopted.current().kid(), newer.current().kid());
    assert_eq!(storage.accepted_writes().await, 0);
}
