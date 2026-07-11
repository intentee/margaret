use margaret_spiffe_svid_manager::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_DNS_ONLY_DER;
use rustls::CertificateError;
use rustls::Error;

#[test]
fn rejects_cert_without_spiffe_san() {
    let result = extract_spiffe_trust_domain(LEAF_DNS_ONLY_DER);

    assert_eq!(
        result.unwrap_err(),
        Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
    );
}
