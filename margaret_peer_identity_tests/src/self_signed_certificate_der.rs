use rcgen::CertificateParams;
use rcgen::KeyPair;
use rcgen::KeyUsagePurpose;
use rcgen::SanType;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn self_signed_certificate_der(subject_alt_names: Vec<SanType>) -> Vec<u8> {
    let mut certificate_params = CertificateParams::default();

    certificate_params.subject_alt_names = subject_alt_names;
    certificate_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];

    let key_pair = KeyPair::generate().expect("the key pair generates");

    certificate_params
        .self_signed(&key_pair)
        .expect("the certificate self-signs")
        .der()
        .to_vec()
}
