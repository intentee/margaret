use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use serde_json::json;

use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen_tests::fixture_pair::fixture_pair;
use margaret_jws_verification::ec_jwk::EcJwk;
use margaret_jws_verification::jwk::Jwk;

#[tokio::test]
async fn persisted_jwks_secret_reads_the_format_of_earlier_releases() -> Result<()> {
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };
    let pair = fixture_pair(Curve::P256, "fixture-kid");
    let token = pair.sign(&claims).await?;
    let Jwk::Ec(EcJwk { x, y, .. }) = pair.public_jwk().clone() else {
        panic!("the pair publishes an ec key");
    };
    let pair_json = json!({
        "signing": { "crv": "P-256", "kid": "fixture-kid", "pem": pair.signing_key().pem().as_str() },
        "public": { "crv": "P-256", "kid": "fixture-kid", "kty": "EC", "use": "sig", "x": x, "y": y }
    });
    let next = fixture_pair(Curve::P256, "next-kid");
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
    let secret: JwksSecret =
        serde_json::from_value::<PersistedJwksSecret>(document)?.into_secret()?;

    assert!(matches!(
        secret.verify_any::<TestClaims>(&token),
        JwksSecretVerificationResult::SignedWithCurrent(verified) if verified == claims
    ));

    Ok(())
}
