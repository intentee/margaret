use std::net::SocketAddr;
use std::sync::Arc;

use reqwest::Client;
use rustls::ClientConfig;

pub struct ServedMesh {
    pub mesh: SocketAddr,
    pub mesh_client: Client,
    pub mesh_tls: Arc<ClientConfig>,
    pub public: SocketAddr,
    pub public_client: Client,
}
