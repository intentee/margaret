use std::sync::Arc;

use rustls::ClientConfig;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;

use margaret_spiffe_svid_client::svid_server_cert_verifier::SvidServerCertVerifier;
use margaret_spiffe_svid_tests::build_root_cert_store_with_ca::build_root_cert_store_with_ca;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn build_mtls_client_config(
    leaf_der: &[u8],
    leaf_key_der: &[u8],
    spiffe_trust_domain: &str,
) -> ClientConfig {
    let cert_chain = vec![CertificateDer::from(leaf_der.to_vec())];
    let private_key = PrivateKeyDer::try_from(leaf_key_der.to_vec()).unwrap();

    ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(
            SvidServerCertVerifier::new(
                build_root_cert_store_with_ca(), spiffe_trust_domain,
            )
            .unwrap(),
        ))
        .with_client_auth_cert(cert_chain, private_key)
        .unwrap()
}
