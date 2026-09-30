use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn jwks_secret_rotation_preserves_the_curve() -> Result<()> {
    let rotated = JwksSecret::fresh(SigningCurve::P384, &FixtureRsaSigningKeys::default())?
        .rotate(&FixtureRsaSigningKeys::default())?;

    assert_eq!(rotated.current().signing_key().curve(), SigningCurve::P384);
    assert_eq!(rotated.next().signing_key().curve(), SigningCurve::P384);

    Ok(())
}
