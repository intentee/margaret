use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwk_signing::JwkSigning;
use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;
use margaret_jwks_key_gen::signs_claims::SignsClaims;
use serde_json::json;

#[tokio::test]
async fn sign_errors_on_invalid_pem() -> Result<()> {
    let signing = JwkSigning {
        crv: Curve::P256,
        kid: "kid".to_string(),
        pem: "not a valid pem".to_string(),
    };

    let result = signing.sign(&json!({})).await;

    assert!(matches!(
        result,
        Err(JwksKeyError::SigningKeyRejected { .. })
    ));

    Ok(())
}
