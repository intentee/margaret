use std::sync::Arc;

use crate::handler_name::HandlerName;
use crate::matchit::InsertError;
use crate::method_handler::MethodHandler;
use crate::named_handler::NamedHandler;
use crate::route_entry::RouteEntry;
use crate::router::Router;

fn named_handlers(entries: &[RouteEntry]) -> Vec<NamedHandler> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            RouteEntry::Http { handlers, .. } => Some(handlers),
            RouteEntry::WebSocket { .. } => None,
        })
        .flatten()
        .filter_map(|MethodHandler { handler, name, .. }| match name {
            HandlerName::Anonymous => None,
            HandlerName::Named(name) => Some(NamedHandler::new(name, Arc::clone(handler))),
        })
        .collect()
}

pub struct ServerRoutes {
    pub named_handlers: Vec<NamedHandler>,
    pub router: Router,
}

impl ServerRoutes {
    /// # Errors
    ///
    /// Returns `InsertError` when two entries claim the same path.
    pub fn build(entries: Vec<RouteEntry>) -> Result<Self, InsertError> {
        let named_handlers = named_handlers(&entries);

        Ok(Self {
            named_handlers,
            router: Router::build(entries)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::method_handler::MethodHandler;
    use crate::route_entry::RouteEntry;

    use super::ServerRoutes;
    use crate::handler::Handler;
    use crate::handler_error::HandlerError;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

    struct Missing;

    #[async_trait::async_trait]
    impl Handler for Missing {
        async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
            Ok(ResponseContinuation::Done(Response::not_found()))
        }
    }

    fn not_found() -> Arc<dyn Handler> {
        Arc::new(Missing)
    }

    #[test]
    fn names_only_the_handlers_that_declare_a_name() {
        let server_routes = ServerRoutes::build(Vec::from([RouteEntry::new(
            "/articles",
            Vec::from([
                MethodHandler::named("GET", "get_articles", not_found()),
                MethodHandler::anonymous("POST", not_found()),
            ]),
        )]))
        .expect("the entries build a router");

        assert_eq!(server_routes.named_handlers.len(), 1);
    }

    #[test]
    fn carries_its_router_when_no_handler_is_named() {
        let server_routes =
            ServerRoutes::build(Vec::new()).expect("an empty entry list builds a router");

        assert!(server_routes.named_handlers.is_empty());
    }
}
