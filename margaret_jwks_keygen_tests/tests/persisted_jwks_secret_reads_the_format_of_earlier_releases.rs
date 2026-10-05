use anyhow::Result;
use serde_json::json;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_pair::fixture_pair;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::ec_jwk::EcJwk;
use margaret_jws_verification::jwk::Jwk;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn persisted_jwks_secret_reads_the_format_of_earlier_releases() -> Result<()> {
    let trust = fixture_trust();
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let pair = fixture_pair(SigningCurve::P256, "fixture-kid");
    let token = claims.signed_by(&pair);
    let Jwk::Ec(EcJwk { x, y, .. }) = pair.public_jwk().clone() else {
        panic!("the pair publishes an ec key");
    };
    let pair_json = json!({
        "signing": { "crv": "P-256", "kid": "fixture-kid", "pem": pair.signing_key().pem().as_str() },
        "public": { "crv": "P-256", "kid": "fixture-kid", "kty": "EC", "use": "sig", "x": x, "y": y }
    });
    let next = fixture_pair(SigningCurve::P256, "next-kid");
    let Jwk::Ec(EcJwk {
        x: next_x,
        y: next_y,
        ..
    }) = next.public_jwk().clone()
    else {
        panic!("the pair publishes an ec key");
    };
    let next_json = json!({
        "signing": { "crv": "P-256", "kid": "next-kid", "pem": next.signing_key().pem().as_str() },
        "public": { "crv": "P-256", "kid": "next-kid", "kty": "EC", "use": "sig", "x": next_x, "y": next_y }
    });
    let document = json!({ "current": pair_json, "next": next_json, "previous": pair_json });
    let secret: JwksSecret = serde_json::from_value::<PersistedJwksSecret>(document)?
        .into_secret(&FixtureRsaSigningKeys::default())?;

    assert!(matches!(
        secret.verify_jwt::<TestClaims, AccessTokenProfile>(&token, &trust.expectation(), NumericDate::new(0)),
        JwksSecretVerificationResult::SignedWithCurrent(verified) if verified.claims == claims
    ));

    Ok(())
}
