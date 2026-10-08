use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;

#[test]
fn persisted_jwks_secret_rejects_an_rsa_key_sharing_the_key_id_of_an_ec_key() {
    let mut document = persisted_document(&fresh_secret(SigningCurve::P256));

    document["rsa"]["next"]["kid"] = document["ec"]["current"]["kid"].clone();

    assert!(matches!(
        restored_document(document),
        Err(JwksKeyError::DuplicateKeyId { .. })
    ));
}
