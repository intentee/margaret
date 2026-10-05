use std::sync::Arc;

use log::error;
use rustls::DigitallySignedStruct;
use rustls::DistinguishedName;
use rustls::Error;
use rustls::SignatureScheme;
use rustls::client::danger::HandshakeSignatureValid;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::UnixTime;
use rustls::server::danger::ClientCertVerified;
use rustls::server::danger::ClientCertVerifier;

use margaret_spiffe_svid::reject_tls12_signature::reject_tls12_signature;
use margaret_sync_holder::sync_holder::SyncHolder;

use crate::svid_client_cert_verifier::SvidClientCertVerifier;

#[derive(Debug, Default)]
pub struct SvidClientCertVerifierFacade {
    current_verifier: SyncHolder<Option<Arc<SvidClientCertVerifier>>>,
}

impl SvidClientCertVerifierFacade {
    pub fn update_internal_verifier(&self, verifier: Arc<SvidClientCertVerifier>) {
        self.current_verifier.set(Some(verifier));
    }

    fn current_verifier(&self) -> Option<Arc<SvidClientCertVerifier>> {
        self.current_verifier.get()
    }
}

impl ClientCertVerifier for SvidClientCertVerifierFacade {
    fn root_hint_subjects(&self) -> &[DistinguishedName] {
        &[]
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        match self.current_verifier() {
            Some(current_verifier) => current_verifier.supported_verify_schemes(),
            None => Vec::new(),
        }
    }

    fn verify_client_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        now: UnixTime,
    ) -> Result<ClientCertVerified, Error> {
        self.current_verifier()
            .ok_or_else(|| {
                error!(
                    "Client request came in, but the server is not ready yet to verify client cert."
                );

                Error::General("Server is not ready yet (SVID client cert)".to_string())
            })?
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
        self.current_verifier()
            .ok_or_else(|| {
                error!(
                    "Client request came in, but the server is not ready yet to verify TLS 1.3 signature."
                );

                Error::General("Server is not ready yet (SVID client cert)".to_string())
            })?
            .verify_tls13_signature(message, cert, dss)
    }
}
