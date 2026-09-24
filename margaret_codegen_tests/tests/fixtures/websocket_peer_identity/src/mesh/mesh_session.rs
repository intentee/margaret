use spiffe::spiffe_id::SpiffeId;

use margaret::framework::macros::build_for_session;
use margaret::framework::macros::middleware;
use margaret::framework::macros::websocket_session;

#[middleware(mesh_peer)]
#[websocket_session(path = "/mesh", server = "mesh")]
pub struct MeshSession {
    peer: SpiffeId,
}

impl MeshSession {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[build_for_session]
    pub fn build_for_session(peer: &SpiffeId) -> anyhow::Result<Self> {
        Ok(Self { peer: peer.clone() })
    }

    #[must_use]
    pub fn peer(&self) -> &SpiffeId {
        &self.peer
    }
}
