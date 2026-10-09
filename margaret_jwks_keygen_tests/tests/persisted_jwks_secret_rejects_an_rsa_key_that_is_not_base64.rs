use serde_json::Value;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;

#[test]
fn persisted_jwks_secret_rejects_an_rsa_key_that_is_not_base64() {
    let persisted = persisted_document(&fresh_secret(SigningCurve::P256));

    for slot in ["current", "next"] {
        let mut document = persisted.clone();

        document["rsa"][slot]["pkcs8"] = Value::String("not base64!".to_string());

        assert!(matches!(
            restored_document(document),
            Err(JwksKeyError::RsaKeyBase64 { .. })
        ));
    }
}
