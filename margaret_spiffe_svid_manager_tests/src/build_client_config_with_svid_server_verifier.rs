use std::sync::Arc;

use margaret_spiffe_svid_manager::svid_server_cert_verifier::SvidServerCertVerifier;
use rustls::ClientConfig;

use crate::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[must_use]
pub fn build_client_config_with_svid_server_verifier() -> ClientConfig {
    ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(
            SvidServerCertVerifier::new(build_root_cert_store_with_ca(), "example.org".to_string())
                .unwrap(),
        ))
        .with_no_client_auth()
}
