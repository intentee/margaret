use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::malformed_rsa_signing_keys::MalformedRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn initial_secret_reports_a_key_generation_error_when_no_rsa_key_can_be_made() {
    let storage = FixtureSigningKeys::empty();

    assert!(matches!(
        initial_secret(&storage, SigningCurve::P256, &MalformedRsaSigningKeys).await,
        Err(RollerError::KeyGeneration(_))
    ));
    assert_eq!(storage.stored_documents().await, 0);
}
