use rustls::CertificateError;
use rustls::Error;

use margaret_peer_identity::peer_identity_error::PeerIdentityError;

#[must_use]
pub fn peer_identity_rejection(error: &Error) -> Option<&PeerIdentityError> {
    match error {
        Error::InvalidCertificate(CertificateError::Other(other)) => {
            other.0.downcast_ref::<PeerIdentityError>()
        }
        _ => None,
    }
}
