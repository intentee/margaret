use anyhow::Result;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair::generate_keypair;
use margaret_jwks_key_gen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::token_verification::TokenVerification;
use margaret_jwks_key_gen::verifies_token::VerifiesToken;
use margaret_jwks_key_gen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[tokio::test]
async fn verify_rejects_wrong_key_signature() -> Result<()> {
    let signing_keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "signer".to_string(),
    })?;
    let other_keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "other".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = signing_keypair.signing.sign(&claims).await?;
    let result = other_keypair.public.verify::<TestClaims>(&token)?;

    assert!(matches!(result, TokenVerification::SignatureMismatch));
    assert!(result.must().is_err());

    Ok(())
}
