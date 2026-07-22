use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_any_token::VerifiesAnyToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn jwks_secret_reports_invalid_for_next_key_token() -> Result<()> {
    let rotated = JwksSecret::fresh(Curve::P256)?.rotate()?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = rotated.next.signing.sign(&claims).await?;

    assert!(matches!(
        rotated.verify_any::<TestClaims>(&token)?,
        JwksSecretVerificationResult::Invalid
    ));

    Ok(())
}
