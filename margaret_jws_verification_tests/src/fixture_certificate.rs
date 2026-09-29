use rcgen::CertificateParams;
use rcgen::KeyPair;

/// # Panics
///
/// Panics when the fixture key cannot be certified.
#[must_use]
pub fn fixture_certificate(pkcs8: &[u8]) -> Vec<u8> {
    let key_pair = KeyPair::try_from(pkcs8).expect("the fixture key is a pkcs#8 key pair");

    CertificateParams::new(vec!["fixture.example".to_string()])
        .expect("the fixture subject name is valid")
        .self_signed(&key_pair)
        .expect("the fixture key signs its own certificate")
        .der()
        .to_vec()
}
