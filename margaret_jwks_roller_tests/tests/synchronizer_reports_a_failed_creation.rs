use std::sync::Arc;

use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::signing_keys_steps::SigningKeysSteps;
use margaret_jwks_roller_tests::stepped_signing_keys::SteppedSigningKeys;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn synchronizer_reports_a_failed_creation() {
    let storage = Arc::new(FixtureSigningKeys::empty());
    let steps = SigningKeysSteps::default();
    let synchronizer = fixture_synchronizer(Arc::new(SteppedSigningKeys {
        inner: storage.clone(),
        steps: steps.clone(),
    }));
    let synchronizing = tokio::spawn(async move {
        synchronizer
            .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
            .await
    });

    steps.pass().await;
    steps.intervene(storage.break_down()).await;

    assert!(matches!(
        synchronizing.await.expect("the synchronization joins"),
        Err(RollerError::SecretCreate { .. })
    ));
}
