use std::sync::Arc;

use rustls::CertificateError;
use rustls::Error;
use rustls::OtherError;
use spiffe::spiffe_id::TrustDomain;

use margaret_peer_identity::spiffe_id_extraction::SpiffeIdExtraction;
use margaret_peer_identity::spiffe_id_from_cert::spiffe_id_from_cert;

/// # Errors
///
/// Returns `Error` propagated from the work it performs.
pub fn extract_spiffe_trust_domain(certificate_der: &[u8]) -> Result<TrustDomain, Error> {
    match spiffe_id_from_cert(certificate_der) {
        SpiffeIdExtraction::Extracted(spiffe_id) => Ok(spiffe_id.trust_domain().clone()),
        SpiffeIdExtraction::Rejected(rejection) => Err(Error::InvalidCertificate(
            CertificateError::Other(OtherError(Arc::new(rejection))),
        )),
    }
}
