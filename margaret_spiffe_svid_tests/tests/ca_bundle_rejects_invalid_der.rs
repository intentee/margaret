use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid::ca_bundle::CaBundle;

#[test]
fn refuses_to_build_a_root_store_from_an_undecodable_ca_so_trust_is_never_partial() {
    let bundle = CaBundle {
        ca_certs: vec![CertificateDer::from(vec![0xFF, 0x00, 0x01])],
    };

    let result = bundle.to_root_cert_store();

    assert!(result.is_err());
}
