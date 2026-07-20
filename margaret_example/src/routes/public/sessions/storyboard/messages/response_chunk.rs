use margaret_macros::websocket_message;
use serde::Serialize;

#[websocket_message(response)]
#[derive(Serialize)]
pub struct ResponseChunk {
    pub text: String,
}
