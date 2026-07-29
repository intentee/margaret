use std::sync::Arc;

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
use rustls::crypto::WebPkiSupportedAlgorithms;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use rustls::pki_types::UnixTime;
use spiffe::spiffe_id::TrustDomain;
use webpki::KeyUsage;

use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid::reject_tls12_signature::reject_tls12_signature;

use crate::parse_end_entity_cert::parse_end_entity_cert;
use crate::svid_error::SvidError;

#[derive(Debug)]
pub struct SvidServerCertVerifier {
    inner_verifier: Arc<WebPkiServerVerifier>,
    root_store: RootCertStore,
    signature_verification_algorithms: WebPkiSupportedAlgorithms,
    spiffe_trust_domain: TrustDomain,
}

impl SvidServerCertVerifier {
    /// # Errors
    ///
    /// Returns `SvidError::CryptoProviderNotInstalled` or `SvidError::ServerVerifier`.
    pub fn new(root_store: RootCertStore, spiffe_trust_domain: &str) -> Result<Self, SvidError> {
        let default_crypto_provider =
            CryptoProvider::get_default().ok_or(SvidError::CryptoProviderNotInstalled)?;
        let spiffe_trust_domain = TrustDomain::new(spiffe_trust_domain)
            .map_err(|source| SvidError::TrustDomain { source })?;
        let signature_verification_algorithms =
            default_crypto_provider.signature_verification_algorithms;
        let inner_verifier: Arc<WebPkiServerVerifier> =
            WebPkiServerVerifier::builder_with_provider(
                Arc::new(root_store.clone()),
                default_crypto_provider.clone(),
            )
            .build()
            .map_err(|source| SvidError::ServerVerifier { source })?;

        Ok(Self {
            inner_verifier,
            root_store,
            signature_verification_algorithms,
            spiffe_trust_domain,
        })
    }
}

impl ServerCertVerifier for SvidServerCertVerifier {
    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.inner_verifier.supported_verify_schemes()
    }

    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
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
                self.signature_verification_algorithms.all,
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
        reject_tls12_signature()
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
}
