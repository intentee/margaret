use margaret::framework::macros::websocket_message;
use serde::Serialize;

#[websocket_message(response, method = "response_chunk")]
#[derive(Serialize)]
pub struct ResponseChunk {
    pub text: String,
}
