use rustls::pki_types::CertificateDer;

use margaret_spiffe_svid::ca_bundle::CaBundle;
use margaret_spiffe_svid_tests::ca_der::CA_DER;
use margaret_spiffe_svid_tests::ca_untrusted_der::CA_UNTRUSTED_DER;

#[test]
fn converts_multiple_cas_to_root_store() {
    let bundle = CaBundle {
        ca_certs: vec![
            CertificateDer::from(CA_DER.to_vec()),
            CertificateDer::from(CA_UNTRUSTED_DER.to_vec()),
        ],
    };

    let root_store = bundle.to_root_cert_store().unwrap();

    assert_eq!(root_store.roots.len(), 2);
}
