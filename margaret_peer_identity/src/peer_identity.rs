use log::warn;
use spiffe::spiffe_id::SpiffeId;

use crate::spiffe_id_extraction::SpiffeIdExtraction;
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
                SpiffeIdExtraction::Extracted(spiffe_id) => Self::Verified { spiffe_id },
                SpiffeIdExtraction::Rejected(rejection) => {
                    warn!("unable to extract a SPIFFE ID from the peer certificate: {rejection}");

                    Self::Unidentified
                }
            },
        }
    }
}
