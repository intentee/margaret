use spiffe::spiffe_id::SpiffeId;

use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::macros::handles_middleware_attribute;
use margaret::framework::macros::process;

#[handles_middleware_attribute(attribute = mesh_peer)]
pub struct MeshPeerGate;

impl MeshPeerGate {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn process(
        &self,
        request: &Request,
        peer: &SpiffeId,
        next: Next,
    ) -> anyhow::Result<ResponseContinuation> {
        if peer.path().is_empty() {
            return Ok(ResponseContinuation::from(Response::forbidden()));
        }

        Ok(next.run(request).await?)
    }
}
