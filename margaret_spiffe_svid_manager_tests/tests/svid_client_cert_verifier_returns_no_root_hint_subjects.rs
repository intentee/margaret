use rustls::server::danger::ClientCertVerifier as _;

use margaret_spiffe_svid_manager::svid_client_cert_verifier::SvidClientCertVerifier;

#[test]
fn returns_no_root_hint_subjects() {
    let verifier = SvidClientCertVerifier::default();

    assert!(verifier.root_hint_subjects().is_empty());
}
