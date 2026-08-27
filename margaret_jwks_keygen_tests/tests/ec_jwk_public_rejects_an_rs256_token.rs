use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::jws_algorithm::JwsAlgorithm;
use margaret_jwks_keygen::token_malformation::TokenMalformation;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::sign_rs256_token::sign_rs256_token;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[test]
fn ec_jwk_public_rejects_an_rs256_token() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "ec-kid".to_string(),
    })?;
    let token = sign_rs256_token(
        "ec-kid",
        &TestClaims {
            exp: FAR_FUTURE_EXPIRY,
            sub: "subject".to_string(),
        },
    );

    assert!(matches!(
        keypair.public.verify::<TestClaims>(&token)?,
        TokenVerification::Malformed(TokenMalformation::AlgorithmMismatch {
            expected: JwsAlgorithm::Es256,
            found: JwsAlgorithm::Rs256,
        })
    ));

    Ok(())
}
