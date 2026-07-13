use anyhow::Result;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair::generate_keypair;
use margaret_jwks_key_gen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;
use margaret_jwks_key_gen::verifies_token::VerifiesToken;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[test]
fn verify_reports_invalid_token_for_garbage() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })?;

    let result = keypair.public.verify::<TestClaims>("not-a-valid-jwt");

    assert!(matches!(result, Err(JwksKeyError::MalformedCompactJws)));

    Ok(())
}
