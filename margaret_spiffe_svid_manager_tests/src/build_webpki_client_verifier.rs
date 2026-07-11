use std::sync::Arc;

use rustls::server::WebPkiClientVerifier;
use rustls::server::danger::ClientCertVerifier;

use crate::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[must_use]
pub fn build_webpki_client_verifier() -> Arc<dyn ClientCertVerifier> {
    WebPkiClientVerifier::builder(Arc::new(build_root_cert_store_with_ca()))
        .build()
        .unwrap()
}
