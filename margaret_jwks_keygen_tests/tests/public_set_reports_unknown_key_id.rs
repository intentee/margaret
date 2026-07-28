use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::token_malformation::TokenMalformation;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn public_set_reports_unknown_key_id() -> Result<()> {
    let present_keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "present".to_string(),
    })?;
    let absent_keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "absent".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = absent_keypair.signing.sign(&claims).await?;
    let set = PublicJwks {
        keys: vec![present_keypair.public],
    };
    let result = set.verify::<TestClaims>(&token);

    assert!(matches!(
        result,
        Ok(TokenVerification::Malformed(
            TokenMalformation::UnknownKeyId { .. }
        ))
    ));

    Ok(())
}
