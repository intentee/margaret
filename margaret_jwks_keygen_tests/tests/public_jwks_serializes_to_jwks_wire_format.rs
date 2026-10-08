use anyhow::Result;
use serde_json::to_value;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jws_verification::ec_jwk::EcJwk;
use margaret_jws_verification::jwk::Jwk;

#[test]
fn public_jwks_serializes_to_jwks_wire_format() -> Result<()> {
    let secret = fresh_secret(SigningCurve::P256);
    let Jwk::Ec(EcJwk { x, y, .. }) = secret.current().public_jwk().clone() else {
        panic!("the pair publishes an ec key");
    };
    let serialized = to_value(secret.public_jwks())?;
    let key = &serialized["keys"][0];

    assert_eq!(key["alg"], "ES256");
    assert_eq!(key["crv"], "P-256");
    assert_eq!(key["kid"], secret.current().kid().as_str());
    assert_eq!(key["kty"], "EC");
    assert_eq!(key["use"], "sig");
    assert_eq!(key["x"], x.as_str());
    assert_eq!(key["y"], y.as_str());

    Ok(())
}
