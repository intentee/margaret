use spiffe::spiffe_id::SpiffeId;

use margaret::framework::http::response::Response;
use margaret::framework::macros::middleware;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

#[singleton]
#[middleware(mesh_gate)]
#[responds_to_http(method = RouteMethod::Get, name = "get_peer", path = "/peer", server = "mesh")]
pub struct GetPeer;

impl GetPeer {
    /// # Errors
    ///
    /// Never returns an error; every verified peer is named.
    #[process]
    pub fn respond(&self, peer: &SpiffeId) -> anyhow::Result<Response> {
        Ok(Response::text(200, peer.to_string()))
    }
}
