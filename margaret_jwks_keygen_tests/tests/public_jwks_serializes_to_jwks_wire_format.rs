use anyhow::Result;
use serde_json::to_value;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::jwk_public::JwkPublic;
use margaret_jwks_keygen::public_jwks::PublicJwks;

#[test]
fn public_jwks_serializes_to_jwks_wire_format() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "wire-kid".to_string(),
    })?;
    let expected_x = keypair.public.x.clone();
    let expected_y = keypair.public.y.clone();
    let set = PublicJwks::new(vec![JwkPublic::Ec(keypair.public)])
        .expect("the key set has unique key ids");

    let serialized = to_value(&set)?;
    let key = &serialized["keys"][0];

    assert_eq!(key["alg"], "ES256");
    assert_eq!(key["crv"], "P-256");
    assert_eq!(key["kid"], "wire-kid");
    assert_eq!(key["kty"], "EC");
    assert_eq!(key["use"], "sig");
    assert_eq!(key["x"], expected_x.as_str());
    assert_eq!(key["y"], expected_y.as_str());

    Ok(())
}
