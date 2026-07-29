use anyhow::Result;

use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::token_malformation::TokenMalformation;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[test]
fn public_set_rejects_malformed_token() -> Result<()> {
    let set = PublicJwks { keys: Vec::new() };

    let result = set.verify::<TestClaims>("garbage");

    assert!(matches!(
        result,
        Ok(TokenVerification::Malformed(
            TokenMalformation::NotCompactJws
        ))
    ));

    Ok(())
}
