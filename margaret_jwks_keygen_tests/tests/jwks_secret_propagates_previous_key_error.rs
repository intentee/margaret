use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_any_token::VerifiesAnyToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn jwks_secret_propagates_previous_key_error() -> Result<()> {
    let mut secret = JwksSecret::fresh(Curve::P256)?.rotate()?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = secret.previous.signing.sign(&claims).await?;
    secret.previous.public.x = "invalid @@@".to_string();

    let result = secret.verify_any::<TestClaims>(&token);

    assert!(matches!(result, Err(JwksKeyError::CoordinateBase64 { .. })));

    Ok(())
}
