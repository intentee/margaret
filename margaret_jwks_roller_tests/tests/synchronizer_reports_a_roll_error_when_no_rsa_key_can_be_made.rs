use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::malformed_rsa_signing_keys::MalformedRsaSigningKeys;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller::signing_keys_synchronizer::SigningKeysSynchronizer;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn synchronizer_reports_a_roll_error_when_no_rsa_key_can_be_made() {
    let held = Arc::new(fresh_secret(SigningCurve::P256));
    let storage = Arc::new(FixtureSigningKeys::storing(&held));
    let synchronizer = SigningKeysSynchronizer {
        curve: SigningCurve::P256,
        rsa_keys: Arc::new(MalformedRsaSigningKeys),
        storage: storage.clone(),
    };

    assert!(matches!(
        synchronizer
            .synchronized(
                &HeldSecret::Held(held.clone()),
                held.rolled_at().after(JWKS_ROLL_INTERVAL)
            )
            .await,
        Err(RollerError::KeyRoll(_))
    ));
    assert_eq!(storage.accepted_writes().await, 0);
}
