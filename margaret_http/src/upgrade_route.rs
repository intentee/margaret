use std::collections::BTreeMap;
use std::sync::Arc;

use crate::http_middleware::HttpMiddleware;
use crate::web_socket_upgrade::WebSocketUpgrade;

pub(crate) struct UpgradeRoute {
    pub(crate) middleware: Vec<Arc<dyn HttpMiddleware>>,
    pub(crate) path_params: BTreeMap<String, String>,
    pub(crate) upgrade: Arc<dyn WebSocketUpgrade>,
}
