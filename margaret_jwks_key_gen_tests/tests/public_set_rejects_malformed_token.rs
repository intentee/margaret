use anyhow::Result;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;
use margaret_jwks_key_gen::verifies_token::VerifiesToken;

use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[test]
fn public_set_rejects_malformed_token() -> Result<()> {
    let set = JwkPublicSet { keys: Vec::new() };

    let result = set.verify::<TestClaims>("garbage");

    assert!(matches!(result, Err(JwksKeyError::MalformedCompactJws)));

    Ok(())
}
