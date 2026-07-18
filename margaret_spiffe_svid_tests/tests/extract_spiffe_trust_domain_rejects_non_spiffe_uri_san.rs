use rustls::CertificateError;
use rustls::Error;

use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_tests::leaf_https_uri_der::LEAF_HTTPS_URI_DER;

#[test]
fn rejects_non_spiffe_uri_san() {
    let result = extract_spiffe_trust_domain(LEAF_HTTPS_URI_DER);

    assert_eq!(
        result.unwrap_err(),
        Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
    );
}
