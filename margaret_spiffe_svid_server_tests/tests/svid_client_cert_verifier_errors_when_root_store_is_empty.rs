use rustls::RootCertStore;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;

#[test]
fn errors_when_root_store_is_empty() {
    let result = SvidClientCertVerifier::new(
        RootCertStore::empty(),
        "example.org",
        svid_crypto_provider(),
    );

    assert!(result.is_err());
}
