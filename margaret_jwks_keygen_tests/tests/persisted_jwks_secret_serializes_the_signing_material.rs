use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;

#[test]
fn persisted_jwks_secret_serializes_the_signing_material() {
    let secret = rolled_secret(&fresh_secret(SigningCurve::P256));
    let serialized = persisted_document(&secret);

    assert_eq!(
        serialized["ec"]["current"]["kid"],
        secret.current().kid().as_str()
    );
    assert_eq!(
        serialized["ec"]["current"]["pem"],
        secret.current().signing_key().pem().as_str()
    );
    assert_eq!(serialized["ec"]["current"]["crv"], "P-256");
    assert_eq!(
        serialized["rolled_at"],
        secret.rolled_at().seconds_since_epoch()
    );
}
