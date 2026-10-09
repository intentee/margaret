use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;

#[test]
fn persisted_jwks_secret_rejects_a_corrupt_signing_key() {
    let persisted = persisted_document(&fresh_secret(SigningCurve::P256));

    for slot in ["current", "next"] {
        let mut document = persisted.clone();

        document["ec"][slot]["pem"] = "not a pkcs8 pem".into();

        assert!(matches!(
            restored_document(document),
            Err(JwksKeyError::SigningKeyRejected { .. })
        ));
    }
}
