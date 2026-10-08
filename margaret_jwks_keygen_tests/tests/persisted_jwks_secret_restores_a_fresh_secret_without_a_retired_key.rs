use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;

#[test]
fn persisted_jwks_secret_restores_a_fresh_secret_without_a_retired_key() {
    let restored = restored_document(persisted_document(&fresh_secret(SigningCurve::P256)))
        .expect("the document restores");

    assert!(restored.retired().is_empty());
    assert!(restored.rsa().retired().is_empty());
}
