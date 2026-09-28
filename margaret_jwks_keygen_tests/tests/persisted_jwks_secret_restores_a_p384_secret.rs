use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;

#[test]
fn persisted_jwks_secret_restores_a_p384_secret() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P384)?;
    let document = serde_json::to_vec(&PersistedJwksSecret::from_secret(&secret))?;
    let restored = serde_json::from_slice::<PersistedJwksSecret>(&document)?.into_secret()?;

    assert_eq!(restored.current().signing_key().curve(), Curve::P384);
    assert_eq!(restored.current().kid(), secret.current().kid());

    Ok(())
}
