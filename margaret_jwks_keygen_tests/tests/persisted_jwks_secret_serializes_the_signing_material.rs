use anyhow::Result;
use serde_json::to_value;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;

#[test]
fn persisted_jwks_secret_serializes_the_signing_material() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;
    let serialized = to_value(PersistedJwksSecret::from_secret(&secret))?;

    assert_eq!(
        serialized["current"]["signing"]["kid"],
        secret.current().kid().as_str()
    );
    assert_eq!(
        serialized["current"]["signing"]["pem"],
        secret.current().signing_key().pem().as_str()
    );
    assert_eq!(serialized["current"]["public"]["kty"], "EC");
    assert_eq!(serialized["previous"]["public"]["crv"], "P-256");
    assert_eq!(serialized["previous"]["public"]["use"], "sig");

    Ok(())
}
