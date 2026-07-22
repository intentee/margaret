use anyhow::Result;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn sign_produces_canonical_low_s_signature() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = keypair.signing.sign(&claims).await?;

    let (_, signature_segment) = token.rsplit_once('.').unwrap();
    let signature =
        p256::ecdsa::Signature::from_slice(&Base64UrlUnpadded::decode_vec(signature_segment)?)?;
    assert!(signature.normalize_s().is_none());

    assert!(matches!(
        keypair.public.verify::<TestClaims>(&token)?,
        TokenVerification::Verified(_)
    ));

    Ok(())
}
