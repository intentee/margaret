use serde::Serialize;

use margaret::framework::macros::websocket_message;

#[websocket_message(response, method = "chat_echo")]
#[derive(Serialize)]
pub struct ChatEcho {
    pub text: String,
}
