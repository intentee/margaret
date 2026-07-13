use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid_manager::ca_bundle::CaBundle;
use margaret_spiffe_svid_manager_tests::test_fixtures::CA_DER;

#[test]
fn converts_single_ca_to_root_store() {
    let bundle = CaBundle {
        ca_certs: vec![CertificateDer::from(CA_DER.to_vec())],
    };

    let root_store = bundle.to_root_cert_store().unwrap();

    assert_eq!(root_store.roots.len(), 1);
}
