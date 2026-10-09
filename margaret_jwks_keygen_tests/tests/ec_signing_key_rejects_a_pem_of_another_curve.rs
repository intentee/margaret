use margaret_jwks_keygen::ec_signing_key::EcSigningKey;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;

#[test]
fn ec_signing_key_rejects_a_pem_of_another_curve() {
    let p384 = EcSigningKey::generate(SigningCurve::P384).expect("the key generates");

    assert!(matches!(
        EcSigningKey::from_pkcs8_pem(SigningCurve::P256, p384.pem().clone()),
        Err(JwksKeyError::SigningKeyRejected { .. })
    ));
}
