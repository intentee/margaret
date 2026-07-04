use crate::named_handler::NamedHandler;
use crate::router::Router;

pub struct ServerRoutes {
    pub named_handlers: Vec<NamedHandler>,
    pub router: Router,
}

impl ServerRoutes {
    pub fn new(router: Router, named_handlers: Vec<NamedHandler>) -> Self {
        Self {
            named_handlers,
            router,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ServerRoutes;
    use crate::router_builder::RouterBuilder;

    #[test]
    fn carries_its_router_and_named_handlers() {
        let server_routes = ServerRoutes::new(RouterBuilder::empty().build(), Vec::new());

        assert!(server_routes.named_handlers.is_empty());
    }
}
