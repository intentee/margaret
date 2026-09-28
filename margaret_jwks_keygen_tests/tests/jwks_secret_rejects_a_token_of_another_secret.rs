use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::jws_rejection::JwsRejection;

#[tokio::test]
async fn jwks_secret_rejects_a_token_of_another_secret() -> Result<()> {
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(Curve::P256)?;
    let token = JwksSecret::fresh(Curve::P256)?
        .current()
        .sign(&claims)
        .await?;

    assert!(matches!(
        secret.verify_any::<TestClaims>(&token),
        JwksSecretVerificationResult::Rejected(JwsRejection::UnknownKeyId { .. })
    ));

    Ok(())
}
