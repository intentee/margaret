use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::verifies_any_token::VerifiesAnyToken;

use margaret_jwks_key_gen_tests::test_claims::FAR_FUTURE_EXPIRY;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[tokio::test]
async fn jwks_secret_verifies_token_signed_with_current() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = secret.current.signing.sign(&claims).await?;
    let result = secret.verify_any::<TestClaims>(&token)?;

    assert!(matches!(
        result,
        JwksSecretVerificationResult::SignedWithCurrent(verified) if verified == claims
    ));

    Ok(())
}
