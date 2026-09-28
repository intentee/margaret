use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn jwks_secret_verifies_a_token_of_the_retired_key_after_rotation() -> Result<()> {
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(Curve::P256)?;
    let token = secret.current().sign(&claims).await?;
    let rotated = secret.rotate()?;

    assert!(matches!(
        rotated.verify_any::<TestClaims>(&token),
        JwksSecretVerificationResult::SignedWithPrevious(verified) if verified == claims
    ));

    Ok(())
}
