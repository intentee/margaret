use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid::ca_bundle::CaBundle;
use margaret_spiffe_svid_tests::constants::CA_DER;
use margaret_spiffe_svid_tests::constants::CA_UNTRUSTED_DER;

#[test]
fn converts_every_ca_into_the_root_store_so_peers_of_each_authority_are_trusted() {
    let bundle = CaBundle {
        ca_certs: vec![
            CertificateDer::from(CA_DER.to_vec()),
            CertificateDer::from(CA_UNTRUSTED_DER.to_vec()),
        ],
    };

    let root_store = bundle.to_root_cert_store().unwrap();

    assert_eq!(root_store.roots.len(), 2);
}
