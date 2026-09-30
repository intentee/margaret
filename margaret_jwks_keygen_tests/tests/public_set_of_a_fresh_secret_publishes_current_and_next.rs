use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn public_set_of_a_fresh_secret_publishes_current_and_next() -> Result<()> {
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())?;

    assert_eq!(
        secret.public_jwks().keys(),
        [
            secret.current().public_jwk().clone(),
            secret.next().public_jwk().clone(),
            secret.rsa().current().public_jwk().clone(),
            secret.rsa().next().public_jwk().clone()
        ]
    );

    Ok(())
}
