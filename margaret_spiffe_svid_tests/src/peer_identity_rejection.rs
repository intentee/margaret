use rustls::CertificateError;
use rustls::Error;

use margaret_peer_identity::spiffe_id_rejection::SpiffeIdRejection;

#[must_use]
pub fn peer_identity_rejection(error: &Error) -> Option<&SpiffeIdRejection> {
    match error {
        Error::InvalidCertificate(CertificateError::Other(other)) => {
            other.0.downcast_ref::<SpiffeIdRejection>()
        }
        _ => None,
    }
}
