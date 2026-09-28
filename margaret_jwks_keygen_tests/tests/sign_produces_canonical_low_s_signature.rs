use anyhow::Result;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::Signature;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::fixture_pair::fixture_pair;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn sign_produces_canonical_low_s_signature() -> Result<()> {
    let keypair = fixture_pair(Curve::P256, "kid");
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = keypair.sign(&claims).await?;

    let (_, signature_segment) = token.rsplit_once('.').expect("the token has a signature");
    let signature = Signature::from_slice(&Base64UrlUnpadded::decode_vec(signature_segment)?)?;
    assert!(signature.normalize_s().is_none());

    Ok(())
}
