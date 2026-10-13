use std::sync::Arc;

use crate::http_middleware::HttpMiddleware;
use crate::method_handler::MethodHandler;
use crate::web_socket_upgrade::WebSocketUpgrade;

pub enum RouteEntry {
    Http {
        handlers: Vec<MethodHandler>,
    },
    WebSocket {
        middleware: Vec<Arc<dyn HttpMiddleware>>,
        upgrade: Arc<dyn WebSocketUpgrade>,
    },
}

impl RouteEntry {
    #[must_use]
    pub fn new(handlers: Vec<MethodHandler>) -> Self {
        Self::Http { handlers }
    }

    #[must_use]
    pub fn web_socket(
        upgrade: Arc<dyn WebSocketUpgrade>,
        middleware: Vec<Arc<dyn HttpMiddleware>>,
    ) -> Self {
        Self::WebSocket {
            middleware,
            upgrade,
        }
    }
}
