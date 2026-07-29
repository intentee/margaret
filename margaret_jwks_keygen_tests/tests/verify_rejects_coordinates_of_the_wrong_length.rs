use anyhow::Result;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding as _;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::jwk_public::JwkPublic;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::key_type::KeyType;
use margaret_jwks_keygen::key_use::KeyUse;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn verify_rejects_coordinates_of_the_wrong_length() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "kid".to_string(),
    })?;
    let token = keypair
        .signing
        .sign(&TestClaims {
            exp: FAR_FUTURE_EXPIRY,
            sub: "subject".to_string(),
        })
        .await?;

    let x = Base64UrlUnpadded::decode_vec(&keypair.public.x)?;
    let y = Base64UrlUnpadded::decode_vec(&keypair.public.y)?;
    let mut shifted_x = x.clone();
    let mut shifted_y = y.clone();

    shifted_x.pop();
    shifted_y.insert(0, 0x00);

    assert_eq!(shifted_x.len() + shifted_y.len(), x.len() + y.len());

    let mis_split_public_key = JwkPublic {
        crv: Curve::P256,
        kid: "kid".to_string(),
        kty: KeyType::Ec,
        use_: KeyUse::Signature,
        x: Base64UrlUnpadded::encode_string(&shifted_x),
        y: Base64UrlUnpadded::encode_string(&shifted_y),
    };

    assert!(matches!(
        mis_split_public_key.verify::<TestClaims>(&token),
        Err(JwksKeyError::CoordinateLength {
            expected: 32,
            found: 31
        })
    ));

    Ok(())
}
