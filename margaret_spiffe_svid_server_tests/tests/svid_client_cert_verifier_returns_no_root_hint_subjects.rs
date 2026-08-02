use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn returns_no_root_hint_subjects() {
    install_crypto_provider();

    let verifier =
        SvidClientCertVerifier::new(build_root_cert_store_with_ca(), "example.org").unwrap();

    assert!(verifier.root_hint_subjects().is_empty());
}
