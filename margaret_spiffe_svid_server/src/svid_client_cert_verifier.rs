use std::sync::Arc;

use log::warn;
use rustls::CertificateError;
use rustls::DigitallySignedStruct;
use rustls::DistinguishedName;
use rustls::Error;
use rustls::RootCertStore;
use rustls::SignatureScheme;
use rustls::client::danger::HandshakeSignatureValid;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::UnixTime;
use rustls::server::WebPkiClientVerifier;
use rustls::server::danger::ClientCertVerified;
use rustls::server::danger::ClientCertVerifier;

use margaret_spiffe_svid::extract_spiffe_trust_domain::extract_spiffe_trust_domain;
use margaret_spiffe_svid::reject_tls12_signature::reject_tls12_signature;

use crate::svid_error::SvidError;

#[derive(Debug)]
pub struct SvidClientCertVerifier {
    inner_verifier: Arc<dyn ClientCertVerifier>,
    spiffe_trust_domain: String,
}

impl SvidClientCertVerifier {
    /// # Errors
    ///
    /// Returns `SvidError::ClientVerifier`.
    pub fn new(root_store: RootCertStore, spiffe_trust_domain: String) -> Result<Self, SvidError> {
        let inner_verifier = WebPkiClientVerifier::builder(Arc::new(root_store))
            .build()
            .map_err(|source| SvidError::ClientVerifier { source })?;

        Ok(Self {
            inner_verifier,
            spiffe_trust_domain,
        })
    }
}

impl ClientCertVerifier for SvidClientCertVerifier {
    fn root_hint_subjects(&self) -> &[DistinguishedName] {
        &[]
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.inner_verifier.supported_verify_schemes()
    }

    fn verify_client_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        now: UnixTime,
    ) -> Result<ClientCertVerified, Error> {
        let trust_domain = extract_spiffe_trust_domain(end_entity.as_ref())?;

        if trust_domain != self.spiffe_trust_domain {
            warn!("Untrusted SPIFFE domain: '{trust_domain}'");

            return Err(Error::InvalidCertificate(
                CertificateError::ApplicationVerificationFailure,
            ));
        }

        self.inner_verifier
            .verify_client_cert(end_entity, intermediates, now)
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
