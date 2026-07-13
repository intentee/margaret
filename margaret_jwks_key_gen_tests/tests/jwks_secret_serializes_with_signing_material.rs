use anyhow::Result;
use serde_json::to_value;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;

#[test]
fn jwks_secret_serializes_with_signing_material() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;
    let current_kid = secret.current.signing.kid.clone();
    let current_pem = secret.current.signing.pem.clone();

    let serialized = to_value(&secret)?;

    assert_eq!(
        serialized["current"]["signing"]["kid"],
        current_kid.as_str()
    );
    assert_eq!(
        serialized["current"]["signing"]["pem"],
        current_pem.as_str()
    );
    assert_eq!(serialized["current"]["public"]["kty"], "EC");
    assert_eq!(serialized["previous"]["public"]["crv"], "P-256");
    assert_eq!(serialized["previous"]["public"]["use"], "sig");

    Ok(())
}
