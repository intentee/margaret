use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_server::svid_client_cert_verifier::SvidClientCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[test]
fn exposes_the_webpki_verify_schemes_so_handshakes_can_negotiate_a_signature() {
    let verifier = SvidClientCertVerifier::new(
        build_root_cert_store_with_ca(),
        "example.org",
        svid_crypto_provider(),
    )
    .unwrap();

    assert!(!verifier.supported_verify_schemes().is_empty());
}
