use anyhow::Result;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair::generate_keypair;
use margaret_jwks_key_gen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::verifies_token::VerifiesToken;
use margaret_jwks_key_gen_tests::test_claims::FAR_FUTURE_EXPIRY;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[tokio::test]
async fn verify_rejects_high_s_malleated_token() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };

    let token = keypair.signing.sign(&claims).await?;
    let (signing_input, signature_segment) = token.rsplit_once('.').unwrap();
    let signature =
        p256::ecdsa::Signature::from_slice(&Base64UrlUnpadded::decode_vec(signature_segment)?)?;
    let (r, s) = signature.split_scalars();
    let malleated_signature = p256::ecdsa::Signature::from_scalars(r, -s)?;
    let malleated = format!(
        "{signing_input}.{}",
        Base64UrlUnpadded::encode_string(&malleated_signature.to_bytes())
    );

    let result = keypair.public.verify::<TestClaims>(&malleated);

    assert!(matches!(result, Err(JwksKeyError::NonCanonicalSignature)));

    Ok(())
}
