use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn initial_secret_rejects_stored_keys_of_another_curve() {
    let stored = JwksSecret::fresh(SigningCurve::P384, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");

    let Err(error) = initial_secret(
        &FixtureSigningKeys::storing(&stored),
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .await
    else {
        panic!("the stored curve differs from the pinned one");
    };

    assert!(matches!(
        error,
        RollerError::CurveMismatch {
            pinned: SigningCurve::P256,
            stored: SigningCurve::P384,
        }
    ));
}
