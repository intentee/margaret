use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::verifies_any_token::VerifiesAnyToken;

use margaret_jwks_key_gen_tests::test_claims::FAR_FUTURE_EXPIRY;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[tokio::test]
async fn jwks_secret_falls_back_to_previous_after_rotation() -> Result<()> {
    let rotated = JwksSecret::fresh(Curve::P256)?.rotate(Curve::P256)?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = rotated.previous.signing.sign(&claims).await?;
    let result = rotated.verify_any::<TestClaims>(&token)?;

    assert!(matches!(
        result,
        JwksSecretVerificationResult::SignedWithPrevious(verified) if verified == claims
    ));

    Ok(())
}
