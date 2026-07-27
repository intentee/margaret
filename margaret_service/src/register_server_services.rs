use crate::server_service::ServerService;

pub fn register_server_services(
    manager: &mut trzcina::ServiceManager,
    server_services: Vec<ServerService>,
) {
    for server_service in server_services {
        manager.register_service(server_service);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_http::forward_targets::ForwardTargets;
    use margaret_http::server_registry::ServerRegistry;

    use super::register_server_services;
    use crate::server_service::ServerService;

    #[test]
    fn registers_each_server_service() {
        let registry = Arc::new(ServerRegistry::new(Vec::new()));
        let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));
        let server_service = ServerService::new(registry, forward_targets, "test");
        let mut manager = trzcina::ServiceManager::default();

        register_server_services(&mut manager, vec![server_service]);
    }
}
