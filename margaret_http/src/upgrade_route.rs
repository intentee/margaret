use std::collections::HashMap;
use std::sync::Arc;

use crate::http_middleware::HttpMiddleware;
use crate::web_socket_upgrade::WebSocketUpgrade;

pub(crate) struct UpgradeRoute<TTarget> {
    pub(crate) middleware: Vec<Arc<dyn HttpMiddleware<TTarget>>>,
    pub(crate) path_params: HashMap<String, String>,
    pub(crate) upgrade: Arc<dyn WebSocketUpgrade>,
}
