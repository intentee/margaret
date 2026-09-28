use anyhow::Result;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::decode_header;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::jwk::KeyAlgorithm;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[test]
fn published_jwks_loads_into_a_standard_jwt_verifier() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let token = claims.signed_by(secret.current());
    let document = serde_json::to_string(secret.public_jwks())?;

    let published: JwkSet = serde_json::from_str(&document)?;
    let kid = decode_header(&token)?
        .kid
        .expect("the token header carries a key id");
    let signing_key = published.find(&kid).expect("the signing key is published");

    assert_eq!(
        signing_key.common.key_algorithm,
        Some(KeyAlgorithm::ES256),
        "a standard verifier reads the algorithm off the published key"
    );

    let verified = decode::<TestClaims>(
        &token,
        &DecodingKey::from_jwk(signing_key)?,
        &Validation::new(Algorithm::ES256),
    )?;

    assert_eq!(verified.claims, claims);

    Ok(())
}
