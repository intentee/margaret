use rustls::CertificateError;
use rustls::Error;

use margaret_spiffe_svid_manager::extract_spiffe_trust_domain::extract_spiffe_trust_domain;

#[test]
fn rejects_invalid_der() {
    let result = extract_spiffe_trust_domain(&[0u8; 4]);

    assert_eq!(
        result.unwrap_err(),
        Error::InvalidCertificate(CertificateError::BadEncoding),
    );
}
