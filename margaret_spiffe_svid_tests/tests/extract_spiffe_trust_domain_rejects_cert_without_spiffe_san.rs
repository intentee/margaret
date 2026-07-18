use rustls::CertificateError;
use rustls::Error;

use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid_tests::leaf_dns_only_der::LEAF_DNS_ONLY_DER;

#[test]
fn rejects_cert_without_spiffe_san() {
    let result = extract_spiffe_trust_domain(LEAF_DNS_ONLY_DER);

    assert_eq!(
        result.unwrap_err(),
        Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
    );
}
