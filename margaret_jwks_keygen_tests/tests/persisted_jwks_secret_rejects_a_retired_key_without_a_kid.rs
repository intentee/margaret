use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;

#[test]
fn persisted_jwks_secret_rejects_a_retired_key_without_a_kid() {
    let persisted = persisted_document(&rolled_secret(&fresh_secret(SigningCurve::P256)));

    for ring in ["ec", "rsa"] {
        let mut document = persisted.clone();

        document[ring]["retired"][0]["public"]
            .as_object_mut()
            .expect("the retired key is published as an object")
            .remove("kid");

        assert!(matches!(
            restored_document(document),
            Err(JwksKeyError::RetiredKeyWithoutKid)
        ));
    }
}
