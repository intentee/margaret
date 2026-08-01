use anyhow::Result;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::decode_header;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::jwk::KeyAlgorithm;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::signing_algorithm::signing_algorithm;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn published_jwks_loads_into_a_standard_jwt_verifier() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };
    let token = secret.current.signing.sign(&claims).await?;
    let document = serde_json::to_string(&PublicJwks::from(secret))?;

    let published: JwkSet = serde_json::from_str(&document)?;
    let kid = decode_header(&token)?
        .kid
        .expect("the token header carries a key id");
    let signing_key = published.find(&kid).expect("the signing key is published");
    let key_algorithm = signing_key
        .common
        .key_algorithm
        .expect("a standard verifier reads the algorithm off the published key");

    assert_eq!(key_algorithm, KeyAlgorithm::ES256);

    let verified = decode::<TestClaims>(
        &token,
        &DecodingKey::from_jwk(signing_key)?,
        &Validation::new(signing_algorithm(key_algorithm)?),
    )?;

    assert_eq!(verified.claims, claims);

    Ok(())
}
