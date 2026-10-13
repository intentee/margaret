use std::net::SocketAddr;

use reqwest::Client;

pub struct ServedFallibleRoles {
    pub address: SocketAddr,
    pub client: Client,
}
