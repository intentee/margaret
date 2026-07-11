use margaret_spiffe_svid_manager::ca_bundle::CaBundle;
use rustls::pki_types::CertificateDer;

#[test]
fn rejects_invalid_der() {
    let bundle = CaBundle {
        ca_certs: vec![CertificateDer::from(vec![0xFF, 0x00, 0x01])],
    };

    let result = bundle.to_root_cert_store();

    assert!(result.is_err());
}
