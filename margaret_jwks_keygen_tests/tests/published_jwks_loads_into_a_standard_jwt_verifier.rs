use anyhow::Result;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::decode_header;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::jwk::KeyAlgorithm;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[test]
fn published_jwks_loads_into_a_standard_jwt_verifier() -> Result<()> {
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())?;
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

    let trust = fixture_trust();
    let mut validation = Validation::new(Algorithm::ES256);

    validation.set_audience(&[trust.audience.as_str()]);
    validation.set_issuer(&[trust.issuer.as_str()]);

    let verified = decode::<TestClaims>(&token, &DecodingKey::from_jwk(signing_key)?, &validation)?;

    assert_eq!(verified.claims, claims);

    Ok(())
}
