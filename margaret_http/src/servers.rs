use std::collections::HashMap;
use std::sync::Arc;

use crate::handler::Handler;
use crate::named_handler::NamedHandler;
use crate::server::Server;

pub struct Servers {
    by_name: HashMap<Arc<str>, Server>,
    forward_index: HashMap<&'static str, Arc<dyn Handler>>,
}

impl Servers {
    pub fn new(servers: Vec<Server>, forward_targets: Vec<NamedHandler>) -> Self {
        let by_name = servers
            .into_iter()
            .map(|server| (server.name().clone(), server))
            .collect();
        let forward_index = forward_targets
            .into_iter()
            .map(|named_handler| (named_handler.name(), named_handler.into_handler()))
            .collect();

        Self {
            by_name,
            forward_index,
        }
    }

    pub fn origin(&self, name: &str) -> Option<Arc<str>> {
        self.by_name.get(name).map(|server| server.origin().clone())
    }

    pub fn server(&self, name: &str) -> Option<&Server> {
        self.by_name.get(name)
    }

    pub(crate) fn forward_target(&self, name: &str) -> Option<Arc<dyn Handler>> {
        self.forward_index.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::Servers;
    use crate::router_builder::RouterBuilder;
    use crate::server::Server;
    use crate::upload_config::UploadConfig;

    fn servers() -> Servers {
        Servers::new(
            vec![Server::new(
                "public",
                "127.0.0.1:8080".to_string(),
                "https://example.test",
                UploadConfig::Disabled,
                RouterBuilder::empty().build(),
            )],
            Vec::new(),
        )
    }

    #[test]
    fn resolves_a_registered_server_origin() {
        assert_eq!(
            servers().origin("public").as_deref(),
            Some("https://example.test")
        );
    }

    #[test]
    fn reports_no_origin_for_an_unknown_server() {
        assert!(servers().origin("missing").is_none());
    }

    #[test]
    fn resolves_a_registered_server_by_name() {
        let servers = servers();

        assert_eq!(
            servers.server("public").map(Server::address),
            Some("127.0.0.1:8080")
        );
    }
}
