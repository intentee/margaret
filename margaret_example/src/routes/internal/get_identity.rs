use spiffe::spiffe_id::SpiffeId;

use margaret_http::response::Response;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = "get", path = "/identity", server = "internal")]
pub struct GetIdentity;

impl GetIdentity {
    #[process]
    pub async fn respond(&self, peer: &SpiffeId) -> Response {
        Response::text(
            200,
            format!("trust_domain={} path={}", peer.trust_domain(), peer.path()),
        )
    }
}
