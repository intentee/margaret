use rustls::CertificateError;
use rustls::Error;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::UnixTime;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_tests::leaf_dns_only_der::LEAF_DNS_ONLY_DER;

#[test]
fn rejects_cert_without_spiffe_san() {
    install_crypto_provider();

    let verifier =
        SvidClientCertVerifier::new(build_root_cert_store_with_ca(), "example.org".to_string())
            .unwrap();

    let result = verifier.verify_client_cert(
        &CertificateDer::from(LEAF_DNS_ONLY_DER.to_vec()),
        &[],
        UnixTime::now(),
    );

    assert_eq!(
        result.unwrap_err(),
        Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
    );
}
