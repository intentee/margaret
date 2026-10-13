use std::net::SocketAddr;

use reqwest::Client;

pub struct ServedLibrary {
    pub address: SocketAddr,
    pub client: Client,
}
