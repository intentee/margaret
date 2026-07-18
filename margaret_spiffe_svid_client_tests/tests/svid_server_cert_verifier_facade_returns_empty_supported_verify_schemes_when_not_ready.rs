use rustls::client::danger::ServerCertVerifier as _;

use margaret_spiffe_svid_client::svid_server_cert_verifier_facade::SvidServerCertVerifierFacade;

#[test]
fn returns_empty_supported_verify_schemes_when_not_ready() {
    let facade = SvidServerCertVerifierFacade::default();

    assert!(facade.supported_verify_schemes().is_empty());
}
