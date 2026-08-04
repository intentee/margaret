use std::sync::Arc;

use reqwest::Client;
use rustls::ClientConfig;
use rustls::RootCertStore;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn http_client_without_trust_store() -> Client {
    let _already_installed = rustls::crypto::aws_lc_rs::default_provider().install_default();

    Client::builder()
        .use_preconfigured_tls(
            ClientConfig::builder()
                .with_root_certificates(Arc::new(RootCertStore::empty()))
                .with_no_client_auth(),
        )
        .build()
        .expect("the http client without a trust store builds")
}
