use std::sync::Arc;

use crate::method_handler::MethodHandler;
use crate::named_handler::NamedHandler;
use crate::route_entry::RouteEntry;
use crate::route_position::RoutePosition;
use crate::router::Router;
use crate::router_error::RouterError;

fn named_handlers(entries: &[RouteEntry]) -> Vec<NamedHandler> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            RouteEntry::Http { handlers } => Some(handlers),
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
    pub(crate) fn build(
        matcher: Arc<matchit::Router<RoutePosition>>,
        paths: Vec<&'static str>,
        entries: Vec<RouteEntry>,
    ) -> Result<Self, RouterError> {
        let named_handlers = named_handlers(&entries);

        Ok(Self {
            named_handlers,
            router: Router::build(matcher, paths, entries)?,
        })
    }
}
