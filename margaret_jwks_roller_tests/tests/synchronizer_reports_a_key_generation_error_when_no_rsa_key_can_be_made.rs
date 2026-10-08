use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::malformed_rsa_signing_keys::MalformedRsaSigningKeys;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller::signing_keys_synchronizer::SigningKeysSynchronizer;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn synchronizer_reports_a_key_generation_error_when_no_rsa_key_can_be_made() {
    let storage = Arc::new(FixtureSigningKeys::empty());
    let synchronizer = SigningKeysSynchronizer {
        curve: SigningCurve::P256,
        rsa_keys: Arc::new(MalformedRsaSigningKeys),
        storage: storage.clone(),
    };

    assert!(matches!(
        synchronizer
            .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
            .await,
        Err(RollerError::KeyGeneration(_))
    ));
    assert_eq!(storage.accepted_writes().await, 0);
}
