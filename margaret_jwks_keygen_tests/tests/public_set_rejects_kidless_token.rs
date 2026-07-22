use anyhow::Result;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[test]
fn public_set_rejects_kidless_token() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let header = Base64UrlUnpadded::encode_string(br#"{"alg":"ES256"}"#);
    let payload = Base64UrlUnpadded::encode_string(&serde_json::to_vec(&claims)?);
    let kidless = format!("{header}.{payload}.c2lnbmF0dXJl");

    let set = PublicJwks {
        keys: vec![keypair.public],
    };
    let result = set.verify::<TestClaims>(&kidless);

    assert!(matches!(result, Err(JwksKeyError::HeaderJson { .. })));

    Ok(())
}
