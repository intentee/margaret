use reqwest::Certificate;
use reqwest::Client;
use reqwest::ClientBuilder;

use crate::fixture_certificate_authority::FixtureCertificateAuthority;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn fixture_client_builder(
    certificate_authority: &FixtureCertificateAuthority,
) -> ClientBuilder {
    Client::builder()
        .use_rustls_tls()
        .tls_built_in_root_certs(false)
        .add_root_certificate(
            Certificate::from_der(certificate_authority.certificate_der())
                .expect("the CA certificate is a valid DER certificate"),
        )
}
