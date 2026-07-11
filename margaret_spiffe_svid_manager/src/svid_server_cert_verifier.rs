use std::sync::Arc;

use anyhow::anyhow;
use log::warn;
use rustls::CertificateError;
use rustls::DigitallySignedStruct;
use rustls::Error;
use rustls::RootCertStore;
use rustls::SignatureScheme;
use rustls::client::WebPkiServerVerifier;
use rustls::client::danger::HandshakeSignatureValid;
use rustls::client::danger::ServerCertVerified;
use rustls::client::danger::ServerCertVerifier;
use rustls::crypto::CryptoProvider;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use rustls::pki_types::UnixTime;
use webpki::KeyUsage;
use webpki::ring::ECDSA_P256_SHA256;
use webpki::ring::ECDSA_P256_SHA384;
use webpki::ring::ECDSA_P384_SHA256;
use webpki::ring::ECDSA_P384_SHA384;
use webpki::ring::ED25519;

use crate::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use crate::parse_end_entity_cert::parse_end_entity_cert;

#[derive(Debug)]
pub struct SvidServerCertVerifier {
    inner_verifier: Arc<WebPkiServerVerifier>,
    root_store: RootCertStore,
    spiffe_trust_domain: String,
}

impl SvidServerCertVerifier {
    pub fn new(root_store: RootCertStore, spiffe_trust_domain: String) -> anyhow::Result<Self> {
        let default_crypto_provider = CryptoProvider::get_default()
            .ok_or_else(|| anyhow!("Default rustls crypto provider is not set"))?;

        let inner_verifier: Arc<WebPkiServerVerifier> =
            WebPkiServerVerifier::builder_with_provider(
                Arc::new(root_store.clone()),
                default_crypto_provider.clone(),
            )
            .build()?;

        Ok(Self {
            inner_verifier,
            root_store,
            spiffe_trust_domain,
        })
    }
}

impl ServerCertVerifier for SvidServerCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        // Ignored for SPIFFE
        _server_name: &ServerName<'_>,
        // Ignored for SPIFFE
        _ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        let end_entity_cert = parse_end_entity_cert(end_entity)?;
        let trust_domain = extract_spiffe_trust_domain(end_entity.as_ref())?;

        if trust_domain != self.spiffe_trust_domain {
            warn!("Untrusted SPIFFE domain: '{trust_domain}'");

            return Err(Error::InvalidCertificate(
                CertificateError::ApplicationVerificationFailure,
            ));
        }

        end_entity_cert
            .verify_for_usage(
                &[
                    ECDSA_P256_SHA256,
                    ECDSA_P256_SHA384,
                    ECDSA_P384_SHA256,
                    ECDSA_P384_SHA384,
                    ED25519,
                ],
                &self.root_store.roots,
                intermediates,
                now,
                KeyUsage::server_auth(),
                None,
                None,
            )
            .map_err(|err| {
                warn!("Certificate not good for usage: {err:#?}");

                Error::InvalidCertificate(CertificateError::BadSignature)
            })?;

        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Err(Error::General("TLS 1.2 is not supported".into()))
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        self.inner_verifier
            .verify_tls13_signature(message, cert, dss)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.inner_verifier.supported_verify_schemes()
    }
}
