use std::sync::Arc;

use rcgen::SanType;
use rcgen::string::Ia5String;
use rustls::ServerConfig;
use rustls::crypto::aws_lc_rs;
use url::Url;

use crate::fixture_certificate_authority::FixtureCertificateAuthority;
use crate::issued_certificate::IssuedCertificate;

pub struct TlsFixture {
    pub certificate_authority: FixtureCertificateAuthority,
    pub server_config: Arc<ServerConfig>,
    pub server_name: String,
}

impl TlsFixture {
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    #[must_use]
    pub fn generate() -> Self {
        let server_name = "localhost".to_string();
        let certificate_authority = FixtureCertificateAuthority::generate();
        let IssuedCertificate {
            certificate_der,
            private_key,
        } = certificate_authority.issue(
            &server_name,
            SanType::DnsName(
                Ia5String::try_from(server_name.as_str())
                    .expect("the server name is a valid IA5 string"),
            ),
        );
        let server_config =
            ServerConfig::builder_with_provider(Arc::new(aws_lc_rs::default_provider()))
                .with_safe_default_protocol_versions()
                .expect("the default provider supports the safe protocol versions")
                .with_no_client_auth()
                .with_single_cert(vec![certificate_der], private_key)
                .expect("the server config builds");

        Self {
            certificate_authority,
            server_config: Arc::new(server_config),
            server_name,
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    #[must_use]
    pub fn url(&self, port: u16, path: &str) -> Url {
        Url::parse(&format!("https://{}:{port}{path}", self.server_name))
            .expect("the fixture url parses")
    }
}
