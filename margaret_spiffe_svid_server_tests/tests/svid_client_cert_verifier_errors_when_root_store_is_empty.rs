use rustls::RootCertStore;

use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::install_crypto_provider::install_crypto_provider;

#[test]
fn errors_when_root_store_is_empty() {
    install_crypto_provider();

    let result = SvidClientCertVerifier::new(RootCertStore::empty(), "example.org".to_string());

    assert!(result.is_err());
}
