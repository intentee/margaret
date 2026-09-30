use std::collections::HashMap;
use std::sync::Arc;

use crate::server::Server;

pub struct ServerRegistry {
    by_name: HashMap<Arc<str>, Arc<Server>>,
}

impl ServerRegistry {
    #[must_use]
    pub fn new(servers: Vec<Server>) -> Self {
        let by_name = servers
            .into_iter()
            .map(|server| (server.name().clone(), Arc::new(server)))
            .collect();

        Self { by_name }
    }

    #[must_use]
    pub fn server(&self, name: &str) -> Option<&Arc<Server>> {
        self.by_name.get(name)
    }
}

#[cfg(test)]
mod tests {
    use margaret_http_uploaded_file::upload_config::UploadConfig;

    use super::ServerRegistry;
    use crate::router::Router;
    use crate::server::Server;
    use crate::transport_config::TransportConfig;

    fn registry() -> ServerRegistry {
        ServerRegistry::new(vec![Server::new(
            "public",
            "127.0.0.1:8080".to_string(),
            TransportConfig::Plain,
            UploadConfig::Disabled,
            Router::build(Vec::new()).expect("an empty router builds"),
        )])
    }

    #[test]
    fn resolves_a_registered_server_by_name() {
        assert_eq!(
            registry().server("public").map(|server| server.address()),
            Some("127.0.0.1:8080")
        );
    }

    #[test]
    fn reports_no_server_for_an_unknown_name() {
        assert!(registry().server("missing").is_none());
    }
}
