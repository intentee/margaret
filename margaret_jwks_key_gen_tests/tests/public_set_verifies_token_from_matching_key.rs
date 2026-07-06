use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair::generate_keypair;
use margaret_jwks_key_gen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::verifies_token::VerifiesToken;

use margaret_jwks_key_gen_tests::test_claims::FAR_FUTURE_EXPIRY;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[tokio::test]
async fn public_set_verifies_token_from_matching_key() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "set-kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = keypair.signing.sign(&claims).await?;
    let set = JwkPublicSet {
        keys: vec![keypair.public],
    };
    let verified: TestClaims = set.verify(&token)?.must()?;

    assert_eq!(verified, claims);

    Ok(())
}
