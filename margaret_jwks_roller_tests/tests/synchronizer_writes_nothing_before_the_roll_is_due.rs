use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn synchronizer_writes_nothing_before_the_roll_is_due() {
    let held = Arc::new(fresh_secret(SigningCurve::P256));
    let storage = Arc::new(FixtureSigningKeys::storing(&held));
    let pending = fixture_synchronizer(storage.clone())
        .synchronized(
            &HeldSecret::Held(held.clone()),
            NumericDate::new(
                held.rolled_at()
                    .after(JWKS_ROLL_INTERVAL)
                    .seconds_since_epoch()
                    - 1,
            ),
        )
        .await
        .expect("the held keys stay current");

    assert_eq!(pending.generation(), held.generation());
    assert_eq!(pending.current().kid(), held.current().kid());
    assert_eq!(storage.accepted_writes().await, 0);
}
