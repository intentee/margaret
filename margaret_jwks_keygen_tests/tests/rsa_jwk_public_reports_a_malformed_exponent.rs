use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::key_use::KeyUse;
use margaret_jwks_keygen::rsa_algorithm::RsaAlgorithm;
use margaret_jwks_keygen::rsa_jwk_public::RsaJwkPublic;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::rsa_test_key_modulus::RSA_TEST_KEY_MODULUS;
use margaret_jwks_keygen_tests::sign_rs256_token::sign_rs256_token;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[test]
fn rsa_jwk_public_reports_a_malformed_exponent() {
    let token = sign_rs256_token(
        "rsa-kid",
        &TestClaims {
            exp: FAR_FUTURE_EXPIRY,
            sub: "subject".to_string(),
        },
    );
    let key = RsaJwkPublic {
        alg: RsaAlgorithm::Rs256,
        e: "not-valid-base64-@@@".to_string(),
        kid: "rsa-kid".to_string(),
        n: RSA_TEST_KEY_MODULUS.to_string(),
        use_: KeyUse::Signature,
    };

    assert!(matches!(
        key.verify::<TestClaims>(&token),
        Err(JwksKeyError::RsaExponentBase64 { .. })
    ));
}
