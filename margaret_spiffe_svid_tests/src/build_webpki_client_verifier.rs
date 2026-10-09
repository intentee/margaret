use std::sync::Arc;

use rustls::server::WebPkiClientVerifier;
use rustls::server::danger::ClientCertVerifier;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;

use crate::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn build_webpki_client_verifier() -> Arc<dyn ClientCertVerifier> {
    WebPkiClientVerifier::builder_with_provider(
        Arc::new(build_root_cert_store_with_ca()),
        svid_crypto_provider(),
    )
    .build()
    .unwrap()
}
