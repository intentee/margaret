use rustls::CertificateError;
use rustls::Error;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::UnixTime;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::constants::LEAF_SPIFFE_OTHER_ORG_SERVER_DER;

#[test]
fn rejects_a_client_svid_of_another_trust_domain_so_foreign_workloads_are_refused() {
    let verifier = SvidClientCertVerifier::new(
        build_root_cert_store_with_ca(),
        "example.org",
        svid_crypto_provider(),
    )
    .unwrap();

    let result = verifier.verify_client_cert(
        &CertificateDer::from(LEAF_SPIFFE_OTHER_ORG_SERVER_DER.to_vec()),
        &[],
        UnixTime::now(),
    );

    assert_eq!(
        result.unwrap_err(),
        Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
    );
}
