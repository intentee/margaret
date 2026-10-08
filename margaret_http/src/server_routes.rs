use crate::method_handler::MethodHandler;
use crate::named_handler::NamedHandler;
use crate::route_entry::RouteEntry;
use crate::router::Router;
use crate::router_error::RouterError;

fn named_handlers(entries: &[RouteEntry]) -> Vec<NamedHandler> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            RouteEntry::Http { handlers, .. } => Some(handlers),
            RouteEntry::WebSocket { .. } => None,
        })
        .flatten()
        .filter_map(MethodHandler::forward_target)
        .collect()
}

pub struct ServerRoutes {
    pub named_handlers: Vec<NamedHandler>,
    pub router: Router,
}

impl ServerRoutes {
    /// # Errors
    ///
    /// Returns `RouterError` when the entries do not build a router.
    pub fn build(entries: Vec<RouteEntry>) -> Result<Self, RouterError> {
        let named_handlers = named_handlers(&entries);

        Ok(Self {
            named_handlers,
            router: Router::build(entries)?,
        })
    }
}
