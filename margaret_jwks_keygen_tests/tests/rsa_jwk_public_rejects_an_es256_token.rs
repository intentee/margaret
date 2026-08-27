use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::jws_algorithm::JwsAlgorithm;
use margaret_jwks_keygen::key_use::KeyUse;
use margaret_jwks_keygen::rsa_jwk_public::RsaJwkPublic;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::token_malformation::TokenMalformation;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::rsa_test_key_exponent::RSA_TEST_KEY_EXPONENT;
use margaret_jwks_keygen_tests::rsa_test_key_modulus::RSA_TEST_KEY_MODULUS;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn rsa_jwk_public_rejects_an_es256_token() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "rsa-kid".to_string(),
    })?;
    let token = keypair
        .signing
        .sign(&TestClaims {
            exp: FAR_FUTURE_EXPIRY,
            sub: "subject".to_string(),
        })
        .await?;
    let key = RsaJwkPublic {
        e: RSA_TEST_KEY_EXPONENT.to_string(),
        kid: "rsa-kid".to_string(),
        n: RSA_TEST_KEY_MODULUS.to_string(),
        use_: KeyUse::Signature,
    };

    assert!(matches!(
        key.verify::<TestClaims>(&token)?,
        TokenVerification::Malformed(TokenMalformation::AlgorithmMismatch {
            expected: JwsAlgorithm::Rs256,
            found: JwsAlgorithm::Es256,
        })
    ));

    Ok(())
}
