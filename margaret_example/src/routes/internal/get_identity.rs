use spiffe::spiffe_id::SpiffeId;

use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

#[singleton]
#[responds_to_http(method = "get", path = "/identity", server = "internal")]
pub struct GetIdentity;

impl GetIdentity {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, peer: &SpiffeId) -> anyhow::Result<Response> {
        Ok({
            Response::text(
                200,
                format!("trust_domain={} path={}", peer.trust_domain(), peer.path()),
            )
        })
    }
}
