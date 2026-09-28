use anyhow::Result;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::Signature;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::token_malformation::TokenMalformation;
use margaret_jwks_keygen::token_verification::TokenVerification;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

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
    let signature = Signature::from_slice(&Base64UrlUnpadded::decode_vec(signature_segment)?)?;
    let (r, s) = signature.split_scalars();
    let malleated_signature = Signature::from_scalars(r, -s)?;
    let malleated = format!(
        "{signing_input}.{}",
        Base64UrlUnpadded::encode_string(&malleated_signature.to_bytes())
    );

    let result = keypair.public.verify::<TestClaims>(&malleated);

    assert!(matches!(
        result,
        Ok(TokenVerification::Malformed(
            TokenMalformation::NonCanonicalSignature
        ))
    ));

    Ok(())
}
