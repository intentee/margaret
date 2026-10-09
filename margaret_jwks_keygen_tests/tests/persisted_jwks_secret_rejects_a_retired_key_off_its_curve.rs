use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;

#[test]
fn persisted_jwks_secret_rejects_a_retired_key_off_its_curve() {
    let mut document = persisted_document(&rolled_secret(&fresh_secret(SigningCurve::P256)));

    document["ec"]["retired"][0]["public"]["x"] =
        document["ec"]["retired"][0]["public"]["y"].clone();

    assert!(matches!(
        restored_document(document),
        Err(JwksKeyError::RetiredKeyRejected {
            reason: IgnoredKeyReason::Material(_)
        })
    ));
}
