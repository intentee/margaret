use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Arc;

use http::Method;
use matchit::Params;

use margaret_route_method::route_method::RouteMethod;

use crate::http_middleware::HttpMiddleware;
use crate::method_handler::MethodHandler;
use crate::request_route::RequestRoute;
use crate::route_entry::RouteEntry;
use crate::route_handler::RouteHandler;
use crate::route_position::RoutePosition;
use crate::route_resolution::RouteResolution;
use crate::router_error::RouterError;
use crate::upgrade_route::UpgradeRoute;
use crate::web_socket_upgrade::WebSocketUpgrade;

fn method_handlers(
    path: &'static str,
    handlers: Vec<MethodHandler>,
) -> Result<HashMap<RouteMethod, RouteHandler>, RouterError> {
    let mut by_method = HashMap::with_capacity(handlers.len());

    for MethodHandler { handler, method } in handlers {
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

struct MatchedTarget<'router, 'path> {
    params: Params<'router, 'path>,
    target: &'router RouteTarget,
}

pub struct Router {
    matcher: Arc<matchit::Router<RoutePosition>>,
    targets: Vec<RouteTarget>,
}

impl Router {
    pub(crate) fn build(
        matcher: Arc<matchit::Router<RoutePosition>>,
        paths: Vec<&'static str>,
        entries: Vec<RouteEntry>,
    ) -> Result<Self, RouterError> {
        let mut targets = Vec::with_capacity(entries.len());

        for (path, entry) in paths.into_iter().zip(entries) {
            targets.push(match entry {
                RouteEntry::Http { handlers } => {
                    RouteTarget::Http(method_handlers(path, handlers)?)
                }
                RouteEntry::WebSocket {
                    middleware,
                    upgrade,
                } => RouteTarget::WebSocket {
                    middleware,
                    upgrade,
                },
            });
        }

        Ok(Self { matcher, targets })
    }

    pub(crate) fn resolve(&self, method: &Method, path: &str) -> RouteResolution {
        let matched = self.matcher.at(path).ok().and_then(|matched| {
            self.targets
                .get(matched.value.index())
                .map(|target| MatchedTarget {
                    params: matched.params,
                    target,
                })
        });
        let Some(MatchedTarget { params, target }) = matched else {
            return RouteResolution::Request(RequestRoute::NotFound);
        };
        let path_params = params
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();

        match target {
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
