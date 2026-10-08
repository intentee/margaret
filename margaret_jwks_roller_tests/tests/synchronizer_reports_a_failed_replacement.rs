use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::signing_keys_steps::SigningKeysSteps;
use margaret_jwks_roller_tests::stepped_signing_keys::SteppedSigningKeys;

#[tokio::test]
async fn synchronizer_reports_a_failed_replacement() {
    let stored = fresh_secret(SigningCurve::P256);
    let storage = Arc::new(FixtureSigningKeys::storing(&stored));
    let steps = SigningKeysSteps::default();
    let synchronizer = fixture_synchronizer(Arc::new(SteppedSigningKeys {
        inner: storage.clone(),
        steps: steps.clone(),
    }));
    let due = stored.rolled_at().after(JWKS_ROLL_INTERVAL);
    let synchronizing =
        tokio::spawn(async move { synchronizer.synchronized(&HeldSecret::Unheld, due).await });

    steps.pass().await;
    steps.intervene(storage.break_down()).await;

    let Err(error) = synchronizing.await.expect("the synchronization joins") else {
        panic!("the replacement cannot be stored");
    };

    assert!(matches!(error, RollerError::SecretReplace { .. }));
    assert_eq!(
        error.to_string(),
        "the application could not replace its stored signing keys: the jwks secret backend is unreachable"
    );
    assert_eq!(storage.accepted_writes().await, 0);
}
