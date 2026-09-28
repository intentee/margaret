use anyhow::Result;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::Signature;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen_tests::fixture_pair::fixture_pair;

#[test]
fn sign_produces_canonical_low_s_signature() -> Result<()> {
    let keypair = fixture_pair(Curve::P256, "kid");

    let token = keypair.sign_json(&json!({ "sub": "subject" }), JwtType::Jwt);

    let (_, signature_segment) = token.rsplit_once('.').expect("the token has a signature");
    let signature = Signature::from_slice(&Base64UrlUnpadded::decode_vec(signature_segment)?)?;
    assert!(signature.normalize_s().is_none());

    Ok(())
}
