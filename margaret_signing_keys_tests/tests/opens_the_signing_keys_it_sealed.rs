use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::fixture_signing_keys_encryption_key::fixture_signing_keys_encryption_key;

#[test]
fn opens_the_signing_keys_it_sealed() {
    let encryption_key = fixture_signing_keys_encryption_key();
    let secret = fresh_secret(SigningCurve::P256);

    assert_eq!(
        serde_json::to_value(
            SigningKeysRevision::sealed(&secret, &encryption_key)
                .expect("the signing keys are sealed")
                .opened(&encryption_key)
                .expect("the signing keys open")
        )
        .expect("the opened signing keys serialize"),
        persisted_document(&secret)
    );
}
