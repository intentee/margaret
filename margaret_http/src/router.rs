use std::collections::HashMap;
use std::sync::Arc;

use http::Method;

use crate::handler::Handler;
use crate::http_middleware::HttpMiddleware;
use crate::method_handler::MethodHandler;
use crate::request_route::RequestRoute;
use crate::route_entry::RouteEntry;
use crate::route_resolution::RouteResolution;
use crate::router_error::RouterError;
use crate::upgrade_route::UpgradeRoute;
use crate::web_socket_upgrade::WebSocketUpgrade;

enum RouteTarget {
    Http(HashMap<Method, Arc<dyn Handler>>),
    WebSocket {
        middleware: Vec<Arc<dyn HttpMiddleware>>,
        origin: http::HeaderValue,
        upgrade: Arc<dyn WebSocketUpgrade>,
    },
}

pub struct Router {
    matcher: matchit::Router<RouteTarget>,
}

impl Router {
    pub fn build(entries: Vec<RouteEntry>) -> Result<Self, RouterError> {
        let mut matcher: matchit::Router<RouteTarget> = matchit::Router::new();

        for entry in entries {
            match entry {
                RouteEntry::Http { handlers, path } => {
                    let mut indexed = HashMap::new();

                    for MethodHandler { handler, method } in handlers {
                        if indexed.insert(method.clone(), handler).is_some() {
                            return Err(RouterError::DuplicateMethod { method, path });
                        }
                    }

                    matcher.insert(path, RouteTarget::Http(indexed))?;
                }
                RouteEntry::WebSocket {
                    middleware,
                    origin,
                    path,
                    upgrade,
                } => {
                    matcher.insert(
                        path,
                        RouteTarget::WebSocket {
                            middleware,
                            origin,
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
            Err(matchit::MatchError::NotFound) => {
                return RouteResolution::Request(RequestRoute::NotFound);
            }
        };
        let path_params = matched
            .params
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();

        match matched.value {
            RouteTarget::Http(handlers) => match handlers.get(method) {
                Some(handler) => RouteResolution::Request(RequestRoute::Handler {
                    handler: handler.clone(),
                    path_params,
                }),
                None => RouteResolution::Request(RequestRoute::MethodNotAllowed),
            },
            RouteTarget::WebSocket {
                middleware,
                origin,
                upgrade,
            } => {
                if method == Method::GET {
                    RouteResolution::Upgrade(UpgradeRoute {
                        middleware: middleware.clone(),
                        origin: origin.clone(),
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
