use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use rustls::crypto::aws_lc_rs::sign::any_supported_type;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::sign::CertifiedKey;

use crate::svid_certified_key::SvidCertifiedKey;

pub fn build_svid_certified_key(
    cert_chain: Vec<CertificateDer<'static>>,
    private_key_bytes: &[u8],
) -> Result<SvidCertifiedKey> {
    let private_key_der = PrivateKeyDer::try_from(private_key_bytes.to_vec())
        .map_err(|err| anyhow!("unsupported private key format: {err}"))?;
    let signing_key = any_supported_type(&private_key_der)?;

    Ok(SvidCertifiedKey {
        cert_chain: cert_chain.clone(),
        certified_key: Arc::new(CertifiedKey::new(cert_chain, signing_key)),
        private_key_der,
    })
}
