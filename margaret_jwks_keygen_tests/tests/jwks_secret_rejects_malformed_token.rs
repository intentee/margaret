use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::verifies_any_token::VerifiesAnyToken;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[test]
fn jwks_secret_rejects_malformed_token() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;

    let result = secret.verify_any::<TestClaims>("garbage");

    assert!(matches!(result, Err(JwksKeyError::MalformedCompactJws)));

    Ok(())
}
