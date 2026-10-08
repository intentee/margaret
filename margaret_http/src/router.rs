use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Arc;

use http::Method;
use matchit::MatchError;

use margaret_route_method::route_method::RouteMethod;

use crate::http_middleware::HttpMiddleware;
use crate::method_handler::MethodHandler;
use crate::request_route::RequestRoute;
use crate::route_entry::RouteEntry;
use crate::route_handler::RouteHandler;
use crate::route_resolution::RouteResolution;
use crate::routed_handler::RoutedHandler;
use crate::router_error::RouterError;
use crate::upgrade_route::UpgradeRoute;
use crate::web_socket_upgrade::WebSocketUpgrade;

fn method_handlers(
    path: &'static str,
    handlers: Vec<MethodHandler>,
) -> Result<HashMap<RouteMethod, RouteHandler>, RouterError> {
    let mut by_method = HashMap::with_capacity(handlers.len());

    for RoutedHandler { handler, method } in handlers.into_iter().map(MethodHandler::into_routed) {
        match by_method.entry(method) {
            Entry::Occupied(_) => return Err(RouterError::DuplicateMethod { method, path }),
            Entry::Vacant(slot) => {
                slot.insert(handler);
            }
        }
    }

    Ok(by_method)
}

enum RouteTarget {
    Http(HashMap<RouteMethod, RouteHandler>),
    WebSocket {
        middleware: Vec<Arc<dyn HttpMiddleware>>,
        upgrade: Arc<dyn WebSocketUpgrade>,
    },
}

pub struct Router {
    matcher: matchit::Router<RouteTarget>,
}

impl Router {
    /// # Errors
    ///
    /// Returns `RouterError::Path` when two entries claim the same path, and
    /// `RouterError::DuplicateMethod` when one path answers a method twice.
    pub fn build(entries: Vec<RouteEntry>) -> Result<Self, RouterError> {
        let mut matcher: matchit::Router<RouteTarget> = matchit::Router::new();

        for entry in entries {
            match entry {
                RouteEntry::Http { handlers, path } => {
                    matcher.insert(path, RouteTarget::Http(method_handlers(path, handlers)?))?;
                }
                RouteEntry::WebSocket {
                    middleware,
                    path,
                    upgrade,
                } => {
                    matcher.insert(
                        path,
                        RouteTarget::WebSocket {
                            middleware,
                            upgrade,
                        },
                    )?;
                }
            }
        }

        Ok(Self { matcher })
    }

    pub(crate) fn resolve(&self, method: &Method, path: &str) -> RouteResolution {
        let matched = match self.matcher.at(path) {
            Ok(matched) => matched,
            Err(MatchError::NotFound) => {
                return RouteResolution::Request(RequestRoute::NotFound);
            }
        };
        let path_params = matched
            .params
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();

        match matched.value {
            RouteTarget::Http(handlers) => {
                match RouteMethod::of(method).and_then(|route_method| handlers.get(&route_method)) {
                    Some(handler) => RouteResolution::Request(RequestRoute::Handler {
                        handler: handler.clone(),
                        path_params,
                    }),
                    None => RouteResolution::Request(RequestRoute::MethodNotAllowed),
                }
            }
            RouteTarget::WebSocket {
                middleware,
                upgrade,
            } => {
                if *method == Method::GET {
                    RouteResolution::Upgrade(UpgradeRoute {
                        middleware: middleware.clone(),
                        path_params,
                        upgrade: upgrade.clone(),
                    })
                } else {
                    RouteResolution::Request(RequestRoute::MethodNotAllowed)
                }
            }
        }
    }
}
