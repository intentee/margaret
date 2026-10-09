use spiffe::spiffe_id::SpiffeId;

use margaret_peer_identity::peer_identity::PeerIdentity;

use crate::request::Request;
use crate::requirement::Requirement;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

#[must_use]
pub fn require_peer_spiffe_id(request: &Request) -> Requirement<&SpiffeId> {
    match request.peer_identity() {
        PeerIdentity::Verified { spiffe_id } => Requirement::Met(spiffe_id),
        PeerIdentity::Anonymous | PeerIdentity::Unidentified => {
            Requirement::Unmet(ResponseContinuation::from(Response::forbidden()))
        }
    }
}
