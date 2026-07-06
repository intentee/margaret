use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair_random_kid::generate_keypair_random_kid;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::verifies_any_token::VerifiesAnyToken;

use margaret_jwks_key_gen_tests::test_claims::FAR_FUTURE_EXPIRY;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[tokio::test]
async fn jwks_secret_reports_invalid_for_stranger_token() -> Result<()> {
    let rotated = JwksSecret::fresh(Curve::P256)?.rotate(Curve::P256)?;
    let stranger = generate_keypair_random_kid(Curve::P256)?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = stranger.signing.sign(&claims).await?;
    let result = rotated.verify_any::<TestClaims>(&token)?;

    assert!(matches!(result, JwksSecretVerificationResult::Invalid));

    Ok(())
}
