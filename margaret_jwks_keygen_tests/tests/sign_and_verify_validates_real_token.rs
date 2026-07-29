use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn sign_and_verify_validates_real_token() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "signing-kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = keypair.signing.sign(&claims).await?;
    let verified: TestClaims = keypair
        .public
        .verify(&token)?
        .verified()
        .expect("the token verifies");

    assert_eq!(verified, claims);

    Ok(())
}
