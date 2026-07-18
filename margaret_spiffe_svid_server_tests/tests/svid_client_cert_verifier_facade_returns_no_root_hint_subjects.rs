use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_server::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;

#[test]
fn returns_no_root_hint_subjects() {
    let facade = SvidClientCertVerifierFacade::default();

    assert!(facade.root_hint_subjects().is_empty());
}
