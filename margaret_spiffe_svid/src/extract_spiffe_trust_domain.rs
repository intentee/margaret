use std::sync::Arc;

use rustls::CertificateError;
use rustls::Error;
use rustls::OtherError;
use spiffe::spiffe_id::TrustDomain;

use margaret_peer_identity::spiffe_id_from_cert::spiffe_id_from_cert;

pub fn extract_spiffe_trust_domain(certificate_der: &[u8]) -> Result<TrustDomain, Error> {
    spiffe_id_from_cert(certificate_der)
        .map(|spiffe_id| spiffe_id.trust_domain().clone())
        .map_err(|source| {
            Error::InvalidCertificate(CertificateError::Other(OtherError(Arc::new(source))))
        })
}
