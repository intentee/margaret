use std::sync::Arc;

use rcgen::SanType;
use rcgen::string::Ia5String;
use rustls::ClientConfig;
use rustls::RootCertStore;
use rustls::ServerConfig;
use rustls::crypto::CryptoProvider;
use rustls::crypto::aws_lc_rs;
use rustls::server::WebPkiClientVerifier;

use crate::fixture_certificate_authority::FixtureCertificateAuthority;
use crate::issued_certificate::IssuedCertificate;

fn client_config_for(
    provider: &Arc<CryptoProvider>,
    roots: &Arc<RootCertStore>,
    certificate_authority: &FixtureCertificateAuthority,
    subject_alt_name: SanType,
) -> Arc<ClientConfig> {
    let IssuedCertificate {
        certificate_der,
        private_key,
    } = certificate_authority.issue("margaret fixture client", subject_alt_name);

    Arc::new(
        ClientConfig::builder_with_provider(Arc::clone(provider))
            .with_safe_default_protocol_versions()
            .expect("the default provider supports the safe protocol versions")
            .with_root_certificates(RootCertStore::clone(roots))
            .with_client_auth_cert(vec![certificate_der], private_key)
            .expect("the client config builds"),
    )
}

pub struct MtlsFixture {
    pub client_config: Arc<ClientConfig>,
    pub client_config_without_spiffe_id: Arc<ClientConfig>,
    pub client_spiffe_id: String,
    pub server_config: Arc<ServerConfig>,
    pub server_name: String,
}

impl MtlsFixture {
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    #[must_use]
    pub fn new() -> Self {
        let provider = Arc::new(aws_lc_rs::default_provider());
        let server_name = "localhost".to_string();
        let client_spiffe_id = "spiffe://example.org/test-client".to_string();
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
        let roots = Arc::new(certificate_authority.root_store());
        let client_verifier =
            WebPkiClientVerifier::builder_with_provider(roots.clone(), Arc::clone(&provider))
                .build()
                .expect("the client verifier builds");
        let server_config = ServerConfig::builder_with_provider(Arc::clone(&provider))
            .with_safe_default_protocol_versions()
            .expect("the default provider supports the safe protocol versions")
            .with_client_cert_verifier(client_verifier)
            .with_single_cert(vec![certificate_der], private_key)
            .expect("the server config builds");
        let client_config = client_config_for(
            &provider,
            &roots,
            &certificate_authority,
            SanType::URI(
                Ia5String::try_from(client_spiffe_id.as_str())
                    .expect("the client SPIFFE id is a valid IA5 string"),
            ),
        );
        let client_config_without_spiffe_id = client_config_for(
            &provider,
            &roots,
            &certificate_authority,
            SanType::DnsName(
                Ia5String::try_from("client.example")
                    .expect("the client DNS name is a valid IA5 string"),
            ),
        );

        Self {
            client_config,
            client_config_without_spiffe_id,
            client_spiffe_id,
            server_config: Arc::new(server_config),
            server_name,
        }
    }
}

impl Default for MtlsFixture {
    fn default() -> Self {
        Self::new()
    }
}
