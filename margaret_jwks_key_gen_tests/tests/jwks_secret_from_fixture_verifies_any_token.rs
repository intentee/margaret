use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::generate_keypair::generate_keypair;
use margaret_jwks_key_gen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use margaret_jwks_key_gen::verifies_any_token::VerifiesAnyToken;
use serde_json::from_value;
use serde_json::json;

use margaret_jwks_key_gen_tests::test_claims::FAR_FUTURE_EXPIRY;
use margaret_jwks_key_gen_tests::test_claims::TestClaims;

#[tokio::test]
async fn jwks_secret_from_fixture_verifies_any_token() -> Result<()> {
    let keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "fixture-kid".to_string(),
    })?;
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };
    let token = keypair.signing.sign(&claims).await?;

    let pem = keypair.signing.pem.clone();
    let x = keypair.public.x.clone();
    let y = keypair.public.y.clone();
    let pair_json = || {
        json!({
            "signing": { "crv": "P-256", "kid": "fixture-kid", "pem": pem.clone() },
            "public": {
                "crv": "P-256",
                "kid": "fixture-kid",
                "kty": "EC",
                "use": "sig",
                "x": x.clone(),
                "y": y.clone(),
            }
        })
    };
    let fixture = json!({ "current": pair_json(), "previous": pair_json() });

    let secret: JwksSecret = from_value(fixture)?;
    let result = secret.verify_any::<TestClaims>(&token)?;

    assert!(matches!(
        result,
        JwksSecretVerificationResult::SignedWithCurrent(verified) if verified == claims
    ));

    Ok(())
}
