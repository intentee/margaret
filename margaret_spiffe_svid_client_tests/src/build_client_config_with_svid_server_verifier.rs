use std::sync::Arc;

use rustls::ClientConfig;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;
use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn build_client_config_with_svid_server_verifier() -> ClientConfig {
    ClientConfig::builder_with_provider(svid_crypto_provider())
        .with_safe_default_protocol_versions()
        .expect("the provider supports the safe default protocol versions")
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(
            SvidServerCertVerifier::new(
                build_root_cert_store_with_ca(),
                "example.org",
                svid_crypto_provider(),
            )
            .unwrap(),
        ))
        .with_no_client_auth()
}
