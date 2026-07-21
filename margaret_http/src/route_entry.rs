use std::sync::Arc;

use crate::http_middleware::HttpMiddleware;
use crate::method_handler::MethodHandler;
use crate::web_socket_upgrade::WebSocketUpgrade;

pub enum RouteEntry {
    Http {
        handlers: Vec<MethodHandler>,
        path: &'static str,
    },
    WebSocket {
        middleware: Vec<Arc<dyn HttpMiddleware>>,
        path: &'static str,
        upgrade: Arc<dyn WebSocketUpgrade>,
    },
}

impl RouteEntry {
    #[must_use]
    pub fn new(path: &'static str, handlers: Vec<MethodHandler>) -> Self {
        Self::Http { handlers, path }
    }

    #[must_use]
    pub fn web_socket(
        path: &'static str,
        upgrade: Arc<dyn WebSocketUpgrade>,
        middleware: Vec<Arc<dyn HttpMiddleware>>,
    ) -> Self {
        Self::WebSocket {
            middleware,
            path,
            upgrade,
        }
    }
}
