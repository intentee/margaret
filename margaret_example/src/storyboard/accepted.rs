use margaret_macros::websocket_message;
use serde::Serialize;

#[derive(Serialize)]
#[websocket_message(method = "conversation.accepted")]
pub struct Accepted {
    pub echoed: String,
}
