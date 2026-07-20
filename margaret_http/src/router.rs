use std::collections::HashMap;
use std::sync::Arc;

use matchit::InsertError;

use crate::handler::Handler;
use crate::method_handler::MethodHandler;
use crate::route_entry::RouteEntry;
use crate::web_socket_upgrade::WebSocketUpgrade;

enum RouteTarget {
    Http(HashMap<&'static str, Arc<dyn Handler>>),
    WebSocket(Arc<dyn WebSocketUpgrade>),
}

pub(crate) enum RequestRoute {
    Handler {
        handler: Arc<dyn Handler>,
        path_params: HashMap<String, String>,
    },
    MethodNotAllowed,
    NotFound,
}

pub(crate) enum RouteResolution {
    Request(RequestRoute),
    Upgrade {
        path_params: HashMap<String, String>,
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
                RouteEntry::WebSocket { path, upgrade } => {
                    matcher.insert(path, RouteTarget::WebSocket(upgrade))?;
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
            RouteTarget::WebSocket(upgrade) => {
                if method == "GET" {
                    RouteResolution::Upgrade {
                        path_params,
                        upgrade: upgrade.clone(),
                    }
                } else {
                    RouteResolution::Request(RequestRoute::MethodNotAllowed)
                }
            }
        }
    }
}
