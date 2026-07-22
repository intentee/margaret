use anyhow::Result;
use serde_json::json;
use zeroize::Zeroizing;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwk_signing::JwkSigning;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signs_claims::SignsClaims;

#[tokio::test]
async fn sign_errors_on_invalid_pem() -> Result<()> {
    let signing = JwkSigning {
        crv: Curve::P256,
        kid: "kid".to_string(),
        pem: Zeroizing::new("not a valid pem".to_string()),
    };

    let result = signing.sign(&json!({})).await;

    assert!(matches!(
        result,
        Err(JwksKeyError::SigningKeyRejected { .. })
    ));

    Ok(())
}
