use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn persisted_jwks_secret_restores_a_p384_secret() -> Result<()> {
    let secret = JwksSecret::fresh(SigningCurve::P384, &FixtureRsaSigningKeys::default())?;
    let document = serde_json::to_vec(&PersistedJwksSecret::from_secret(&secret))?;
    let restored = serde_json::from_slice::<PersistedJwksSecret>(&document)?
        .into_secret(&FixtureRsaSigningKeys::default())?;

    assert_eq!(restored.current().signing_key().curve(), SigningCurve::P384);
    assert_eq!(restored.current().kid(), secret.current().kid());

    Ok(())
}
