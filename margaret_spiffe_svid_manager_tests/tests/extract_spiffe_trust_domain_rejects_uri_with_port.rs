use rustls::CertificateError;
use rustls::Error;

use margaret_spiffe_svid_manager::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_manager_tests::test_fixtures::LEAF_SPIFFE_WITH_PORT_DER;

#[test]
fn rejects_spiffe_uri_with_port() {
    let result = extract_spiffe_trust_domain(LEAF_SPIFFE_WITH_PORT_DER);

    assert_eq!(
        result.unwrap_err(),
        Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
    );
}
