use anyhow::Context as _;
use anyhow::Result;
use rustls::RootCertStore;
use rustls::pki_types::CertificateDer;

#[derive(Debug)]
pub struct CaBundle<'bundle> {
    pub ca_certs: Vec<CertificateDer<'bundle>>,
}

impl CaBundle<'_> {
    pub fn to_root_cert_store(&self) -> Result<RootCertStore> {
        let mut root_store = RootCertStore::empty();

        for ca_cert in &self.ca_certs {
            root_store
                .add(ca_cert.clone())
                .context("failed to add CA cert to root store")?;
        }

        Ok(root_store)
    }
}
