use serde::Serialize;

use margaret::framework::macros::websocket_message;

#[websocket_message(response, method = "response_chunk")]
#[derive(Serialize)]
pub struct ResponseChunk {
    pub text: String,
}
