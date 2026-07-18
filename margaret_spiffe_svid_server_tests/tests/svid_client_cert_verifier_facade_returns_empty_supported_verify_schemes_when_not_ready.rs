use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_server::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;

#[test]
fn returns_empty_supported_verify_schemes_when_not_ready() {
    let facade = SvidClientCertVerifierFacade::default();

    assert!(facade.supported_verify_schemes().is_empty());
}
