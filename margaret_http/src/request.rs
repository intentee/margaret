use std::collections::HashMap;
use std::sync::Arc;

use margaret_peer_identity::peer_identity::PeerIdentity;

use crate::request_inputs::RequestInputs;
use crate::server::Server;

pub struct Request {
    pub inputs: RequestInputs,
    path_params: HashMap<String, String>,
    peer_identity: Arc<PeerIdentity>,
    server: Arc<Server>,
}

impl Request {
    #[must_use]
    pub fn new(
        server: Arc<Server>,
        inputs: RequestInputs,
        path_params: HashMap<String, String>,
        peer_identity: Arc<PeerIdentity>,
    ) -> Self {
        Self {
            inputs,
            path_params,
            peer_identity,
            server,
        }
    }

    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(String::as_str)
    }

    #[must_use]
    pub fn peer_identity(&self) -> &PeerIdentity {
        &self.peer_identity
    }

    #[must_use]
    pub fn server(&self) -> &Server {
        &self.server
    }
}
