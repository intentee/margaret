use rustls::RootCertStore;
use rustls::pki_types::CertificateDer;

use crate::test_fixtures::CA_DER;

#[must_use]
pub fn build_root_cert_store_with_ca() -> RootCertStore {
    let mut root_store = RootCertStore::empty();

    root_store
        .add(CertificateDer::from(CA_DER.to_vec()))
        .unwrap();

    root_store
}
