use zeroize::Zeroizing;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::rsa_signing_key::RsaSigningKey;

#[test]
fn rejects_a_pkcs8_document_that_is_not_an_rsa_key() {
    assert!(matches!(
        RsaSigningKey::from_pkcs8(Zeroizing::new(vec![0; 8])),
        Err(JwksKeyError::RsaKeyRejected { .. })
    ));
}
