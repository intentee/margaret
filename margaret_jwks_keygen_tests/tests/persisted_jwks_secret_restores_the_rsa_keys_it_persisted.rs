use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;

#[test]
fn persisted_jwks_secret_restores_the_rsa_keys_it_persisted() {
    let rolled = rolled_secret(&fresh_secret(SigningCurve::P256));
    let restored = restored_document(persisted_document(&rolled)).expect("the document restores");

    assert_eq!(restored.rsa().current().kid(), rolled.rsa().current().kid());
    assert_eq!(
        restored.rsa().current().signing_key().pkcs8(),
        rolled.rsa().current().signing_key().pkcs8()
    );
    assert_eq!(restored.rsa().next().kid(), rolled.rsa().next().kid());
    assert_eq!(
        restored.rsa().retired()[0].public_jwk(),
        rolled.rsa().retired()[0].public_jwk()
    );
    assert_eq!(restored.public_jwks().keys(), rolled.public_jwks().keys());
}
