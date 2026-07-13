use log::warn;
use spiffe::spiffe_id::SpiffeId;

use crate::spiffe_id_from_cert::spiffe_id_from_cert;

pub enum PeerIdentity {
    Anonymous,
    Unidentified,
    Verified { spiffe_id: SpiffeId },
}

impl PeerIdentity {
    #[must_use]
    pub fn from_peer_certificate(certificate_der: Option<&[u8]>) -> Self {
        match certificate_der {
            None => Self::Anonymous,
            Some(certificate_der) => match spiffe_id_from_cert(certificate_der) {
                Ok(spiffe_id) => Self::Verified { spiffe_id },
                Err(error) => {
                    warn!("unable to extract a SPIFFE ID from the peer certificate: {error}");

                    Self::Unidentified
                }
            },
        }
    }
}
