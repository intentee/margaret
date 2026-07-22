use anyhow::Result;
use serde_json::from_value;
use serde_json::json;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn public_jwks_from_fixture_verifies_real_token() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "fixture-kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };
    let token = keypair.signing.sign(&claims).await?;

    let fixture = json!({
        "keys": [{
            "crv": "P-256",
            "kid": "fixture-kid",
            "kty": "EC",
            "use": "sig",
            "x": keypair.public.x,
            "y": keypair.public.y,
        }]
    });

    let set: PublicJwks = from_value(fixture)?;
    let verified: TestClaims = set.verify(&token)?.must()?;

    assert_eq!(verified, claims);

    Ok(())
}
