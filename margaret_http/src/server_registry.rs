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

    pub fn origin(&self, name: &str) -> Option<Arc<str>> {
        self.by_name.get(name).map(|server| server.origin().clone())
    }

    pub fn server(&self, name: &str) -> Option<&Server> {
        self.by_name.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::ServerRegistry;
    use crate::body_limit::BodyLimit;
    use crate::router_builder::RouterBuilder;
    use crate::server::Server;
    use crate::upload_config::UploadConfig;

    fn registry() -> ServerRegistry {
        ServerRegistry::new(vec![Server::new(
            "public",
            "127.0.0.1:8080".to_string(),
            "https://example.test",
            UploadConfig::Disabled,
            BodyLimit::default(),
            RouterBuilder::empty().build(),
        )])
    }

    #[test]
    fn resolves_a_registered_server_origin() {
        assert_eq!(
            registry().origin("public").as_deref(),
            Some("https://example.test")
        );
    }

    #[test]
    fn reports_no_origin_for_an_unknown_server() {
        assert!(registry().origin("missing").is_none());
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
