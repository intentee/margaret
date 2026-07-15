use rustls::pki_types::CertificateDer;
use rustls::pki_types::UnixTime;
use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_manager::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_manager_tests::build_webpki_client_verifier::build_webpki_client_verifier;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;
use margaret_spiffe_svid_manager_tests::leaf_spiffe_example_org_client_der::LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER;

#[test]
fn delegates_after_verifier_update() {
    install_crypto_provider();

    let verifier = SvidClientCertVerifier::default();
    verifier.update_internal_verifier(build_webpki_client_verifier());

    let result = verifier.verify_client_cert(
        &CertificateDer::from(LEAF_SPIFFE_EXAMPLE_ORG_CLIENT_DER.to_vec()),
        &[],
        UnixTime::now(),
    );

    assert!(result.is_ok());
}
