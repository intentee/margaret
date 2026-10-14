use std::sync::Arc;

use crate::http_middleware::HttpMiddleware;
use crate::method_handler::MethodHandler;
use crate::web_socket_upgrade::WebSocketUpgrade;

pub enum RouteEntry<TTarget> {
    Http {
        handlers: Vec<MethodHandler<TTarget>>,
    },
    WebSocket {
        middleware: Vec<Arc<dyn HttpMiddleware<TTarget>>>,
        upgrade: Arc<dyn WebSocketUpgrade>,
    },
}

impl<TTarget> RouteEntry<TTarget> {
    #[must_use]
    pub fn new(handlers: Vec<MethodHandler<TTarget>>) -> Self {
        Self::Http { handlers }
    }

    #[must_use]
    pub fn web_socket(
        upgrade: Arc<dyn WebSocketUpgrade>,
        middleware: Vec<Arc<dyn HttpMiddleware<TTarget>>>,
    ) -> Self {
        Self::WebSocket {
            middleware,
            upgrade,
        }
    }
}
