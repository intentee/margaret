use std::sync::Arc;

use log::error;
use rustls::DigitallySignedStruct;
use rustls::DistinguishedName;
use rustls::SignatureScheme;
use rustls::client::danger::HandshakeSignatureValid;
use rustls::pki_types::CertificateDer;
use rustls::server::danger::ClientCertVerified;
use rustls::server::danger::ClientCertVerifier;

use margaret_sync_holder::sync_holder::SyncHolder;

#[derive(Debug, Default)]
pub struct SvidClientCertVerifier {
    current_verifier: SyncHolder<Arc<dyn ClientCertVerifier>>,
}

impl SvidClientCertVerifier {
    pub fn update_internal_verifier(&self, verifier: Arc<dyn ClientCertVerifier>) {
        self.current_verifier.set(Some(verifier));
    }

    fn current_verifier(&self) -> Option<Arc<dyn ClientCertVerifier>> {
        self.current_verifier.get()
    }
}

impl ClientCertVerifier for SvidClientCertVerifier {
    fn root_hint_subjects(&self) -> &[DistinguishedName] {
        &[]
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.current_verifier()
            .map(|current_verifier| current_verifier.supported_verify_schemes())
            .unwrap_or_default()
    }

    fn verify_client_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        now: rustls::pki_types::UnixTime,
    ) -> std::result::Result<ClientCertVerified, rustls::Error> {
        self.current_verifier()
            .ok_or_else(|| {
                error!(
                    "Client request came in, but the server is not ready yet to verify client cert."
                );

                rustls::Error::General("Server is not ready yet (SVID client cert)".into())
            })?
            .verify_client_cert(end_entity, intermediates, now)
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        error!("Client tried to use TLS 1.2");

        Err(rustls::Error::General(
            "TLS 1.2 is not supported".to_string(),
        ))
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        self.current_verifier()
            .ok_or_else(|| {
                error!("Client request came in, but the server is not ready yet to verify TLS 1.3 signature.");

                rustls::Error::General("Server is not ready yet (SVID client cert)".into())
            })?
            .verify_tls13_signature(message, cert, dss)
    }
}
