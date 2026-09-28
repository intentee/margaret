use zeroize::Zeroizing;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::ec_signing_key::EcSigningKey;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[test]
fn ec_signing_key_rejects_a_pem_that_is_not_a_key() {
    assert!(matches!(
        EcSigningKey::from_pkcs8_pem(Curve::P256, Zeroizing::new("not a pem".to_string())),
        Err(JwksKeyError::SigningKeyRejected { .. })
    ));
}
