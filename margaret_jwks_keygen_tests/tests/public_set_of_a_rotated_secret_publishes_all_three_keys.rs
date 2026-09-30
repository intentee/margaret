use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn public_set_of_a_rotated_secret_publishes_all_three_keys() -> Result<()> {
    let fresh = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())?;
    let rotated = fresh.rotate(&FixtureRsaSigningKeys::default())?;

    assert_eq!(
        rotated.public_jwks().keys(),
        [
            rotated.current().public_jwk().clone(),
            fresh.current().public_jwk().clone(),
            rotated.next().public_jwk().clone(),
            rotated.rsa().current().public_jwk().clone(),
            fresh.rsa().current().public_jwk().clone(),
            rotated.rsa().next().public_jwk().clone()
        ]
    );

    Ok(())
}
