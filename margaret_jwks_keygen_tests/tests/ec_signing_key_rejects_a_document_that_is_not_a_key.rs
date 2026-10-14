use zeroize::Zeroizing;

use margaret_jwks_keygen::ec_signing_key::EcSigningKey;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;

#[test]
fn ec_signing_key_rejects_a_document_that_is_not_a_key() {
    assert!(matches!(
        EcSigningKey::from_pkcs8(SigningCurve::P256, Zeroizing::new(b"not a key".to_vec())),
        Err(JwksKeyError::SigningKeyRejected { .. })
    ));
}
