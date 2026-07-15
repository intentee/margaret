use std::sync::Arc;

use rustls::ClientConfig;
use rustls::RootCertStore;

use crate::svid_certified_key::SvidCertifiedKey;
use crate::svid_error::SvidError;
use crate::svid_server_cert_verifier::SvidServerCertVerifier;

pub fn build_rustls_client_config(
    root_store: RootCertStore,
    svid_certified_key: Arc<SvidCertifiedKey>,
    spiffe_trust_domain: String,
) -> Result<ClientConfig, SvidError> {
    let svid_server_cert_verifier = SvidServerCertVerifier::new(root_store, spiffe_trust_domain)?;

    let client_config = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(svid_server_cert_verifier))
        .with_client_auth_cert(
            svid_certified_key.cert_chain.clone(),
            svid_certified_key.private_key_der.clone_key(),
        )
        .map_err(|source| SvidError::ClientConfig { source })?;

    Ok(client_config)
}
