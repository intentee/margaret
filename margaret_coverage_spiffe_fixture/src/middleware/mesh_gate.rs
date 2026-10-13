use spiffe::spiffe_id::SpiffeId;

use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::macros::handles_middleware_attribute;
use margaret::framework::macros::process;

#[handles_middleware_attribute(attribute = mesh_gate)]
pub struct MeshGate;

impl MeshGate {
    /// # Errors
    ///
    /// Returns an error when the gated route fails to answer.
    #[process]
    pub async fn process(
        &self,
        request: &Request,
        _peer: &SpiffeId,
        next: Next,
    ) -> anyhow::Result<ResponseContinuation> {
        next.run(request).await.map_err(anyhow::Error::from)
    }
}
