use margaret::framework::macros::websocket_message;
use serde::Serialize;

#[websocket_message(response, method = "board_chunk")]
#[derive(Serialize)]
pub struct BoardChunk {
    pub text: String,
}
