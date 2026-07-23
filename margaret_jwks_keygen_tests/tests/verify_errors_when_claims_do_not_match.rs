use anyhow::Result;
use serde::Serialize;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

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
