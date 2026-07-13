use std::sync::Arc;

use rustls::ServerConfig;

pub enum TransportConfig {
    Plain,
    MutualTls { server_config: Arc<ServerConfig> },
}
