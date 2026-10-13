use std::future::Ready;
use std::future::ready;

use spiffe::spiffe_id::SpiffeId;

use margaret_http::requirement::Requirement;

pub fn met_verified_peer(peer: &SpiffeId) -> Ready<Requirement<SpiffeId>> {
    ready(Requirement::Met(peer.clone()))
}
