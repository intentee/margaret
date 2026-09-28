use std::collections::HashMap;
use std::sync::Arc;

use matchit::InsertError;
use matchit::MatchError;

use crate::handler::Handler;
use crate::http_middleware::HttpMiddleware;
use crate::method_handler::MethodHandler;
use crate::request_route::RequestRoute;
use crate::route_entry::RouteEntry;
use crate::route_resolution::RouteResolution;
use crate::upgrade_route::UpgradeRoute;
use crate::web_socket_upgrade::WebSocketUpgrade;

enum RouteTarget {
    Http(HashMap<&'static str, Arc<dyn Handler>>),
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
    /// Returns `InsertError` propagated from the work it performs.
    pub fn build(entries: Vec<RouteEntry>) -> Result<Self, InsertError> {
        let mut matcher: matchit::Router<RouteTarget> = matchit::Router::new();

        for entry in entries {
            match entry {
                RouteEntry::Http { handlers, path } => {
                    matcher.insert(
                        path,
                        RouteTarget::Http(
                            handlers
                                .into_iter()
                                .map(
                                    |MethodHandler {
                                         handler, method, ..
                                     }| (method, handler),
                                )
                                .collect(),
                        ),
                    )?;
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

    pub(crate) fn resolve(&self, method: &str, path: &str) -> RouteResolution {
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
            RouteTarget::Http(handlers) => match handlers.get(method) {
                Some(handler) => RouteResolution::Request(RequestRoute::Handler {
                    handler: handler.clone(),
                    path_params,
                }),
                None => RouteResolution::Request(RequestRoute::MethodNotAllowed),
            },
            RouteTarget::WebSocket {
                middleware,
                upgrade,
            } => {
                if method == "GET" {
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
