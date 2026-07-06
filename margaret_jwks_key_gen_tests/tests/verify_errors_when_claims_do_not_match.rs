use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair::generate_keypair;
use margaret_jwks_key_gen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::verifies_token::VerifiesToken;
use serde::Serialize;

use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[derive(Serialize)]
struct ExpirylessClaims {
    sub: String,
}

#[tokio::test]
async fn verify_errors_when_claims_do_not_match() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })?;

    let token = keypair
        .signing
        .sign(&ExpirylessClaims {
            sub: "subject".to_string(),
        })
        .await?;

    let result = keypair.public.verify::<TestClaims>(&token);

    assert!(matches!(result, Err(JwksKeyError::ClaimsJson { .. })));

    Ok(())
}
