use std::collections::HashMap;
use std::sync::Arc;

use matchit::InsertError;

use crate::decode_url_path_parameter::decode_url_path_parameter;
use crate::handler::Handler;
use crate::http_middleware::HttpMiddleware;
use crate::method_handler::MethodHandler;
use crate::request_route::RequestRoute;
use crate::route_entry::RouteEntry;
use crate::route_parameter_decoding_outcome::RouteParameterDecodingOutcome;
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
        let mut path_params: HashMap<String, String> = HashMap::new();

        for (name, value) in matched.params.iter() {
            match decode_url_path_parameter(value) {
                RouteParameterDecodingOutcome::Decoded(decoded) => {
                    path_params.insert(name.to_string(), decoded);
                }
                RouteParameterDecodingOutcome::NotUtf8 => {
                    return RouteResolution::Request(RequestRoute::NotFound);
                }
            }
        }

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
