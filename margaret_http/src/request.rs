use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use http::request::Parts;

use margaret_peer_identity::peer_identity::PeerIdentity;

use crate::request_inputs::RequestInputs;
use crate::request_outcome::RequestOutcome;
use crate::server::Server;
use crate::server_params::ServerParams;

pub struct Request {
    pub inputs: RequestInputs,
    path_params: HashMap<String, String>,
    peer_identity: Arc<PeerIdentity>,
    server: Arc<Server>,
}

impl Request {
    #[must_use]
    pub fn from_head(
        server: Arc<Server>,
        parts: Parts,
        remote_addr: SocketAddr,
        peer_identity: Arc<PeerIdentity>,
    ) -> RequestOutcome<Self> {
        let server_params = match ServerParams::from_parts(parts, remote_addr) {
            RequestOutcome::Parsed(server_params) => server_params,
            RequestOutcome::Rejected(rejection) => return RequestOutcome::Rejected(rejection),
        };

        match RequestInputs::parse(server_params) {
            RequestOutcome::Parsed(inputs) => RequestOutcome::Parsed(Self {
                inputs,
                path_params: HashMap::new(),
                peer_identity,
                server,
            }),
            RequestOutcome::Rejected(rejection) => RequestOutcome::Rejected(rejection),
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

    #[must_use]
    pub fn with_path_params(self, path_params: HashMap<String, String>) -> Self {
        Self {
            inputs: self.inputs,
            path_params,
            peer_identity: self.peer_identity,
            server: self.server,
        }
    }
}
