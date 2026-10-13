use spiffe::spiffe_id::SpiffeId;

use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

#[websocket_session(path = "/talk", server = "mesh")]
pub struct MeshTalkSession {
    peer: String,
}

impl MeshTalkSession {
    /// # Errors
    ///
    /// Never returns an error; every verified peer may talk on the mesh.
    #[build_for_session]
    pub fn build_for_session(peer: &SpiffeId) -> anyhow::Result<Self> {
        Ok(Self {
            peer: peer.to_string(),
        })
    }

    #[must_use]
    pub fn peer(&self) -> &str {
        &self.peer
    }
}
