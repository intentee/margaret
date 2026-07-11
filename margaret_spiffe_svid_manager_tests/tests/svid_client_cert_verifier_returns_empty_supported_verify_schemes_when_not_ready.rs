use margaret_spiffe_svid_manager::svid_client_cert_verifier::SvidClientCertVerifier;
use rustls::server::danger::ClientCertVerifier as _;

#[test]
fn returns_empty_supported_verify_schemes_when_not_ready() {
    let verifier = SvidClientCertVerifier::default();

    assert!(verifier.supported_verify_schemes().is_empty());
}
