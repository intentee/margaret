use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn initial_secret_rejects_a_malformed_document() {
    let Err(error) = initial_secret(
        &FixtureSigningKeys::holding("not a signing keys document"),
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .await
    else {
        panic!("the malformed document is rejected");
    };

    assert!(matches!(error, RollerError::DocumentMalformed { .. }));
}
