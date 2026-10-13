use reqwest::Certificate;
use reqwest::Client;
use reqwest::redirect::Policy;

use margaret_http_tests::fixture_certificate_authority::FixtureCertificateAuthority;

/// # Panics
///
/// Panics when the client cannot trust the cluster certificate authority.
#[must_use]
pub fn cluster_client(certificate_authority: &FixtureCertificateAuthority) -> Client {
    Client::builder()
        .add_root_certificate(
            Certificate::from_der(certificate_authority.certificate_der())
                .expect("the cluster certificate authority is a certificate"),
        )
        .redirect(Policy::none())
        .build()
        .expect("the cluster client builds")
}
