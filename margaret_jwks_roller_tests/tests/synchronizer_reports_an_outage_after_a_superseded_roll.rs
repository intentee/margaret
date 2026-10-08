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
async fn synchronizer_reports_an_outage_after_a_superseded_roll() {
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
    steps
        .intervene(async {
            fixture_synchronizer(storage.clone())
                .synchronized(&HeldSecret::Unheld, due)
                .await
                .expect("the peer rolls the keys");
        })
        .await;
    steps.intervene(storage.break_down()).await;

    assert!(matches!(
        synchronizing.await.expect("the synchronization joins"),
        Err(RollerError::SecretLoad { .. })
    ));
}
