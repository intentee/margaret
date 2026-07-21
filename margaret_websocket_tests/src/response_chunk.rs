use serde::Serialize;

use margaret_websocket::web_socket_response_message::WebSocketResponseMessage;

#[derive(Serialize)]
pub struct ResponseChunk {
    pub text: String,
}

impl WebSocketResponseMessage for ResponseChunk {
    const METHOD: &'static str = "response_chunk";
}
