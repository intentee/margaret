use anyhow::Result;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::token_malformation::TokenMalformation;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[test]
fn verify_rejects_hmac_algorithm_confusion() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "attacker".to_string(),
    };

    let header = Base64UrlUnpadded::encode_string(br#"{"alg":"HS256","kid":"kid"}"#);
    let payload = Base64UrlUnpadded::encode_string(&serde_json::to_vec(&claims)?);
    let forged = format!("{header}.{payload}.c2lnbmF0dXJl");

    let result = keypair.public.verify::<TestClaims>(&forged);

    assert!(matches!(
        result,
        Ok(TokenVerification::Malformed(TokenMalformation::HeaderJson(
            _
        )))
    ));

    Ok(())
}
