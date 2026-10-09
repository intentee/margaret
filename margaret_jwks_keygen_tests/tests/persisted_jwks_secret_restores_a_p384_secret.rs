use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;

#[test]
fn persisted_jwks_secret_restores_a_p384_secret() {
    let secret = fresh_secret(SigningCurve::P384);
    let restored = restored_document(persisted_document(&secret)).expect("the document restores");

    assert_eq!(restored.current().signing_key().curve(), SigningCurve::P384);
    assert_eq!(restored.current().kid(), secret.current().kid());
}
