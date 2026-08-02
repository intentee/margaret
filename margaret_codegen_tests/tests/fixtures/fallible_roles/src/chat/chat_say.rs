use serde::Deserialize;
use validator::Validate;

use margaret::framework::macros::websocket_message;

#[websocket_message(request, method = "chat_say", response = stream)]
#[derive(Deserialize, Validate)]
pub struct ChatSay {
    #[validate(length(min = 1))]
    pub text: String,
}
