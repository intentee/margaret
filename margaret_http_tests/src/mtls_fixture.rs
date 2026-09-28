use std::sync::Arc;

use rcgen::BasicConstraints;
use rcgen::CertificateParams;
use rcgen::IsCa;
use rcgen::Issuer;
use rcgen::KeyPair;
use rcgen::KeyUsagePurpose;
use rcgen::SanType;
use rcgen::SigningKey;
use rcgen::string::Ia5String;
use rustls::ClientConfig;
use rustls::RootCertStore;
use rustls::ServerConfig;
use rustls::crypto::aws_lc_rs;
use rustls::pki_types::PrivateKeyDer;
use rustls::server::WebPkiClientVerifier;

fn client_config_for(
    roots: &Arc<RootCertStore>,
    issuer: &Issuer<'_, impl SigningKey>,
    subject_alt_name: SanType,
) -> Arc<ClientConfig> {
    let client_key = KeyPair::generate().expect("the client key pair generates");
    let mut client_params = CertificateParams::default();
    client_params.subject_alt_names = vec![subject_alt_name];
    let client_certificate = client_params
        .signed_by(&client_key, issuer)
        .expect("the client certificate is signed by the CA")
        .der()
        .clone();

    Arc::new(
        ClientConfig::builder()
            .with_root_certificates(RootCertStore::clone(roots))
            .with_client_auth_cert(
                vec![client_certificate],
                PrivateKeyDer::Pkcs8(client_key.serialize_der().into()),
            )
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
        let _already_installed = aws_lc_rs::default_provider().install_default();

        let server_name = "localhost".to_string();
        let client_spiffe_id = "spiffe://example.org/test-client".to_string();

        let certificate_authority_key = KeyPair::generate().expect("the CA key pair generates");
        let mut certificate_authority_params = CertificateParams::default();
        certificate_authority_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
        certificate_authority_params.key_usages = vec![
            KeyUsagePurpose::KeyCertSign,
            KeyUsagePurpose::DigitalSignature,
        ];
        let certificate_authority = certificate_authority_params
            .self_signed(&certificate_authority_key)
            .expect("the CA certificate self-signs");
        let issuer = Issuer::from_params(&certificate_authority_params, &certificate_authority_key);

        let server_key = KeyPair::generate().expect("the server key pair generates");
        let mut server_params = CertificateParams::default();
        server_params.subject_alt_names = vec![SanType::DnsName(
            Ia5String::try_from(server_name.as_str())
                .expect("the server name is a valid IA5 string"),
        )];
        let server_certificate = server_params
            .signed_by(&server_key, &issuer)
            .expect("the server certificate is signed by the CA")
            .der()
            .clone();

        let mut roots = RootCertStore::empty();
        roots
            .add(certificate_authority.der().clone())
            .expect("the CA certificate is added to the root store");
        let roots = Arc::new(roots);

        let client_verifier = WebPkiClientVerifier::builder(roots.clone())
            .build()
            .expect("the client verifier builds");
        let server_config = ServerConfig::builder()
            .with_client_cert_verifier(client_verifier)
            .with_single_cert(
                vec![server_certificate],
                PrivateKeyDer::Pkcs8(server_key.serialize_der().into()),
            )
            .expect("the server config builds");

        let client_config = client_config_for(
            &roots,
            &issuer,
            SanType::URI(
                Ia5String::try_from(client_spiffe_id.as_str())
                    .expect("the client SPIFFE id is a valid IA5 string"),
            ),
        );
        let client_config_without_spiffe_id = client_config_for(
            &roots,
            &issuer,
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
