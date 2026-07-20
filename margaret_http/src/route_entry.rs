use std::sync::Arc;

use crate::method_handler::MethodHandler;
use crate::web_socket_upgrade::WebSocketUpgrade;

pub enum RouteEntry {
    Http {
        handlers: Vec<MethodHandler>,
        path: &'static str,
    },
    WebSocket {
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
    pub fn web_socket(path: &'static str, upgrade: Arc<dyn WebSocketUpgrade>) -> Self {
        Self::WebSocket { path, upgrade }
    }
}
