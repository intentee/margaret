use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::unreachable_signing_keys::UnreachableSigningKeys;

#[tokio::test]
async fn initial_secret_reports_a_load_error_when_the_backend_is_unreachable() {
    let Err(error) = initial_secret(
        &UnreachableSigningKeys,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .await
    else {
        panic!("the backend cannot be read");
    };

    assert!(matches!(error, RollerError::SecretLoad { .. }));
    assert_eq!(
        error.to_string(),
        "the application could not load its stored signing keys: the jwks secret backend is unreachable"
    );
}
