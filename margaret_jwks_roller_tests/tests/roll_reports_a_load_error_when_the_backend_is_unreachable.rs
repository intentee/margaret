use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::roll::roll;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::unreachable_jwks_secret_storage::UnreachableJwksSecretStorage;

#[test]
fn roll_reports_a_load_error_when_the_backend_is_unreachable() {
    let storage = UnreachableJwksSecretStorage;
    let holder = JwksSecretHolder::default();

    let Err(error) = roll(
        &storage,
        &holder,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    ) else {
        panic!("the backend cannot be read");
    };

    assert!(matches!(error, RollerError::SecretLoad { .. }));
    assert_eq!(
        error.to_string(),
        "failed to load the persisted jwks secret: the jwks secret backend is unreachable"
    );
    assert!(holder.get().is_none());
}
