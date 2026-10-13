use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid::ca_bundle::CaBundle;
use margaret_spiffe_svid_tests::constants::CA_DER;

#[test]
fn converts_a_single_ca_to_a_root_store_of_one_so_its_peers_are_trusted() {
    let bundle = CaBundle {
        ca_certs: vec![CertificateDer::from(CA_DER.to_vec())],
    };

    let root_store = bundle.to_root_cert_store().unwrap();

    assert_eq!(root_store.roots.len(), 1);
}
