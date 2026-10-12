use rustls::RootCertStore;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;

#[test]
fn refuses_to_build_from_an_empty_root_store_since_no_client_could_be_trusted() {
    let result = SvidClientCertVerifier::new(
        RootCertStore::empty(),
        "example.org",
        svid_crypto_provider(),
    );

    assert!(result.is_err());
}
