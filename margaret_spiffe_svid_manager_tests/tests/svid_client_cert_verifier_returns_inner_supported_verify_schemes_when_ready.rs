use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_manager::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_manager_tests::build_webpki_client_verifier::build_webpki_client_verifier;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn returns_inner_supported_verify_schemes_when_ready() {
    install_crypto_provider();

    let verifier = SvidClientCertVerifier::default();
    verifier.update_internal_verifier(build_webpki_client_verifier());

    assert!(!verifier.supported_verify_schemes().is_empty());
}
