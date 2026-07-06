use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::verifies_any_token::VerifiesAnyToken;

use margaret_jwks_key_gen_tests::test_claims::FAR_FUTURE_EXPIRY;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[tokio::test]
async fn jwks_secret_propagates_previous_key_error() -> Result<()> {
    let mut secret = JwksSecret::fresh(Curve::P256)?.rotate(Curve::P256)?;
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
