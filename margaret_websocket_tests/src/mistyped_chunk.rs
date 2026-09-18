use serde::Deserialize;

use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;

#[derive(Debug, Deserialize)]
pub struct MistypedChunk {
    pub text: u64,
}

impl WebSocketResponseMessage for MistypedChunk {
    const METHOD: &'static str = "response_chunk";
}
