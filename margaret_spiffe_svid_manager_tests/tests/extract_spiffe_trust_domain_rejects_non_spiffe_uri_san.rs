use margaret_spiffe_svid_manager::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_HTTPS_URI_DER;
use rustls::CertificateError;
use rustls::Error;

#[test]
fn rejects_non_spiffe_uri_san() {
    let result = extract_spiffe_trust_domain(LEAF_HTTPS_URI_DER);

    assert_eq!(
        result.unwrap_err(),
        Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
    );
}
