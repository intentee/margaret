use anyhow::Result;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair::generate_keypair;
use margaret_jwks_key_gen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_key_gen::jwk_public::JwkPublic;
use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;
use margaret_jwks_key_gen::key_type::KeyType;
use margaret_jwks_key_gen::key_use::KeyUse;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::verifies_token::VerifiesToken;
use margaret_jwks_key_gen_tests::test_claims::FAR_FUTURE_EXPIRY;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[tokio::test]
async fn verify_reports_malformed_coordinate() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };
    let token = keypair.signing.sign(&claims).await?;

    let malformed_public_key = JwkPublic {
        crv: Curve::P256,
        kid: "kid".to_string(),
        kty: KeyType::Ec,
        use_: KeyUse::Signature,
        x: "not-valid-base64-@@@".to_string(),
        y: "not-valid-base64-@@@".to_string(),
    };

    let result = malformed_public_key.verify::<TestClaims>(&token);

    assert!(matches!(result, Err(JwksKeyError::CoordinateBase64 { .. })));

    Ok(())
}
