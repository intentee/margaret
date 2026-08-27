use anyhow::Result;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding as _;
use serde_json::to_vec;

use margaret_jwks_keygen::key_use::KeyUse;
use margaret_jwks_keygen::rsa_jwk_public::RsaJwkPublic;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::rsa_test_key_exponent::RSA_TEST_KEY_EXPONENT;
use margaret_jwks_keygen_tests::rsa_test_key_modulus::RSA_TEST_KEY_MODULUS;
use margaret_jwks_keygen_tests::sign_rs256_token::sign_rs256_token;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[test]
fn rsa_jwk_public_reports_a_signature_mismatch_for_a_tampered_payload() -> Result<()> {
    let token = sign_rs256_token(
        "rsa-kid",
        &TestClaims {
            exp: FAR_FUTURE_EXPIRY,
            sub: "subject".to_string(),
        },
    );
    let (header_segment, rest) = token.split_once('.').expect("the token has three segments");
    let (_, signature_segment) = rest.split_once('.').expect("the token has three segments");
    let tampered_payload = Base64UrlUnpadded::encode_string(&to_vec(&TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "impostor".to_string(),
    })?);
    let tampered = format!("{header_segment}.{tampered_payload}.{signature_segment}");
    let key = RsaJwkPublic {
        e: RSA_TEST_KEY_EXPONENT.to_string(),
        kid: "rsa-kid".to_string(),
        n: RSA_TEST_KEY_MODULUS.to_string(),
        use_: KeyUse::Signature,
    };

    assert!(matches!(
        key.verify::<TestClaims>(&tampered)?,
        TokenVerification::SignatureMismatch
    ));

    Ok(())
}
