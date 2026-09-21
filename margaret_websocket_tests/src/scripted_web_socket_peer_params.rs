use std::sync::Arc;

use rustls::ServerConfig;

use crate::scripted_peer_closing::ScriptedPeerClosing;
use crate::scripted_peer_step::ScriptedPeerStep;

pub struct ScriptedWebSocketPeerParams {
    pub closing: ScriptedPeerClosing,
    pub script: Vec<ScriptedPeerStep>,
    pub server_config: Arc<ServerConfig>,
}
