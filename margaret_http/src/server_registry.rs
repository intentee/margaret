use std::collections::HashMap;
use std::sync::Arc;

use crate::server::Server;

pub struct ServerRegistry {
    by_name: HashMap<Arc<str>, Server>,
}

impl ServerRegistry {
    pub fn new(servers: Vec<Server>) -> Self {
        let by_name = servers
            .into_iter()
            .map(|server| (server.name().clone(), server))
            .collect();

        Self { by_name }
    }

    pub fn server(&self, name: &str) -> Option<&Server> {
        self.by_name.get(name)
    }
}

#[cfg(test)]
mod tests {
    use margaret_cookie_jar::server_cookies::ServerCookies;

    use super::ServerRegistry;
    use crate::body_limit::BodyLimit;
    use crate::router::Router;
    use crate::server::Server;
    use crate::transport_config::TransportConfig;
    use crate::upload_config::UploadConfig;

    fn registry() -> ServerRegistry {
        ServerRegistry::new(vec![Server::new(
            "public",
            "127.0.0.1:8080".to_string(),
            TransportConfig::Plain,
            UploadConfig::Disabled,
            BodyLimit::default(),
            Router::build(Vec::new()).expect("an empty router builds"),
            ServerCookies::Absent,
        )])
    }

    #[test]
    fn resolves_a_registered_server() {
        assert_eq!(
            registry().server("public").map(Server::address),
            Some("127.0.0.1:8080")
        );
    }

    #[test]
    fn reports_no_server_for_an_unknown_name() {
        assert!(registry().server("missing").is_none());
    }

    #[test]
    fn resolves_a_registered_server_by_name() {
        let registry = registry();

        assert_eq!(
            registry.server("public").map(Server::address),
            Some("127.0.0.1:8080")
        );
    }
}
