use rustls::client::danger::ServerCertVerifier as _;

use margaret_spiffe_svid_manager::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_manager_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_manager_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn returns_inner_supported_verify_schemes() {
    install_crypto_provider();

    let verifier =
        SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "example.org".to_string())
            .unwrap();

    assert!(!verifier.supported_verify_schemes().is_empty());
}
