use std::collections::HashMap;
use std::sync::Arc;

use matchit::InsertError;

use crate::handler::Handler;
use crate::http_middleware::HttpMiddleware;
use crate::method_handler::MethodHandler;
use crate::path_params_decoding::PathParamsDecoding;
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
                                .map(|MethodHandler { handler, method }| (method, handler))
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
            Err(matchit::MatchError::NotFound) => {
                return RouteResolution::Request(RequestRoute::NotFound);
            }
        };
        let path_params = match PathParamsDecoding::from_params(&matched.params) {
            PathParamsDecoding::Decoded(path_params) => path_params,
            PathParamsDecoding::NotValidUtf8 { parameter, source } => {
                return RouteResolution::Request(RequestRoute::PathParameterNotValidUtf8 {
                    parameter,
                    source,
                });
            }
        };

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
