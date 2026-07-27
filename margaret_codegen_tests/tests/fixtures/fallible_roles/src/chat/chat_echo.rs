use margaret::framework::macros::websocket_message;
use serde::Serialize;

#[websocket_message(response, method = "chat_echo")]
#[derive(Serialize)]
pub struct ChatEcho {
    pub text: String,
}
