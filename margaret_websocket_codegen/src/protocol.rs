use margaret_attributes::canonical_path::CanonicalPath;

use crate::websocket_state::WebsocketState;
use crate::websocket_transition::WebsocketTransition;

#[derive(Debug)]
pub struct Protocol<'model> {
    pub entry: CanonicalPath,
    pub entry_field: String,
    pub server: String,
    pub path: String,
    pub states: Vec<&'model WebsocketState>,
    pub transitions: Vec<&'model WebsocketTransition>,
}
