use std::sync::Arc;

use log::error;
use rustls::DigitallySignedStruct;
use rustls::Error;
use rustls::SignatureScheme;
use rustls::client::danger::HandshakeSignatureValid;
use rustls::client::danger::ServerCertVerified;
use rustls::client::danger::ServerCertVerifier;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use rustls::pki_types::UnixTime;

use margaret_spiffe_svid::reject_tls12_signature::reject_tls12_signature;
use margaret_sync_holder::sync_holder::SyncHolder;

use crate::svid_server_cert_verifier::SvidServerCertVerifier;

#[derive(Debug, Default)]
pub struct SvidServerCertVerifierFacade {
    current_verifier: SyncHolder<Arc<SvidServerCertVerifier>>,
}

impl SvidServerCertVerifierFacade {
    pub fn update_internal_verifier(&self, verifier: Arc<SvidServerCertVerifier>) {
        self.current_verifier.set(Some(verifier));
    }

    fn current_verifier(&self) -> Option<Arc<SvidServerCertVerifier>> {
        self.current_verifier.get()
    }
}

impl ServerCertVerifier for SvidServerCertVerifierFacade {
    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.current_verifier()
            .map(|current_verifier| current_verifier.supported_verify_schemes())
            .unwrap_or_default()
    }

    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        self.current_verifier()
            .ok_or_else(|| {
                error!(
                    "Outbound connection attempted, but the client is not ready yet to verify the server cert."
                );

                Error::General("Client is not ready yet (SVID server cert)".to_string())
            })?
            .verify_server_cert(end_entity, intermediates, server_name, ocsp_response, now)
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
        self.current_verifier()
            .ok_or_else(|| {
                error!(
                    "Outbound connection attempted, but the client is not ready yet to verify the TLS 1.3 signature."
                );

                Error::General("Client is not ready yet (SVID server cert)".to_string())
            })?
            .verify_tls13_signature(message, cert, dss)
    }
}
