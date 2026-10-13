use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_server::svid_client_cert_verifier_facade::SvidClientCertVerifierFacade;

#[test]
fn advertises_no_root_hint_subjects_without_a_verifier_so_clients_send_any_svid() {
    let facade = SvidClientCertVerifierFacade::default();

    assert!(facade.root_hint_subjects().is_empty());
}
