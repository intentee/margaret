use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::previous_key::PreviousKey;

#[test]
fn persisted_jwks_secret_restores_a_fresh_secret_without_a_retired_key() -> Result<()> {
    let document = serde_json::to_vec(&PersistedJwksSecret::from_secret(&JwksSecret::fresh(
        Curve::P256,
    )?))?;
    let restored = serde_json::from_slice::<PersistedJwksSecret>(&document)?.into_secret()?;

    assert!(matches!(restored.previous(), PreviousKey::Absent));

    Ok(())
}
