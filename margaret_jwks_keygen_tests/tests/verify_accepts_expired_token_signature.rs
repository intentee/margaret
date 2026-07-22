use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::already_expired_expiry::ALREADY_EXPIRED_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn verify_accepts_expired_token_signature() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: ALREADY_EXPIRED_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = keypair.signing.sign(&claims).await?;
    let verified: TestClaims = keypair.public.verify::<TestClaims>(&token)?.must()?;

    assert_eq!(verified, claims);

    Ok(())
}
