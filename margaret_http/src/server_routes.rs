use matchit::InsertError;

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
    /// Returns `InsertError` when two entries claim the same path.
    pub fn build(entries: Vec<RouteEntry>) -> Result<Self, InsertError> {
        let named_handlers = named_handlers(&entries);

        Ok(Self {
            named_handlers,
            router: Router::build(entries)?,
        })
    }
}
