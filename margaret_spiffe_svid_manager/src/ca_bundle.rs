use rustls::RootCertStore;
use rustls::pki_types::CertificateDer;

use crate::svid_error::SvidError;

#[derive(Debug)]
pub struct CaBundle<'bundle> {
    pub ca_certs: Vec<CertificateDer<'bundle>>,
}

impl CaBundle<'_> {
    pub fn to_root_cert_store(&self) -> Result<RootCertStore, SvidError> {
        let mut root_store = RootCertStore::empty();

        for ca_cert in &self.ca_certs {
            root_store
                .add(ca_cert.clone())
                .map_err(|source| SvidError::RootStoreRejectedCaCert { source })?;
        }

        Ok(root_store)
    }
}
