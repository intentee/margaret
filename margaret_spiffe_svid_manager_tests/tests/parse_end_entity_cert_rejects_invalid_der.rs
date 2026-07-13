use rustls::CertificateError;
use rustls::Error;
use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid_manager::parse_end_entity_cert::parse_end_entity_cert;

#[test]
fn rejects_invalid_der() {
    let cert = CertificateDer::from(vec![0u8; 4]);

    let error = parse_end_entity_cert(&cert).err().unwrap();

    assert_eq!(
        error,
        Error::InvalidCertificate(CertificateError::BadEncoding),
    );
}
