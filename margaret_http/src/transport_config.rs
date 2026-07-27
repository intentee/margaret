use std::sync::Arc;

use rustls::ServerConfig;

pub enum TransportConfig {
    #[cfg(any(test, feature = "fixture-plain-transport"))]
    FixturePlain,
    MutualTls {
        server_config: Arc<ServerConfig>,
    },
}
