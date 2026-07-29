use rustls::ServerConfig;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;

use crate::build_webpki_client_verifier::build_webpki_client_verifier;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn build_mtls_server_config(leaf_der: &[u8], leaf_key_der: &[u8]) -> ServerConfig {
    let cert_chain = vec![CertificateDer::from(leaf_der.to_vec())];
    let private_key = PrivateKeyDer::try_from(leaf_key_der.to_vec()).unwrap();

    ServerConfig::builder()
        .with_client_cert_verifier(build_webpki_client_verifier())
        .with_single_cert(cert_chain, private_key)
        .unwrap()
}
