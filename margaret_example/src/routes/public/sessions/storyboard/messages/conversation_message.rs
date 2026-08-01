use serde::Deserialize;
use validator::Validate;

use margaret::framework::macros::websocket_message;

#[websocket_message(request, method = "conversation_message", response = stream)]
#[derive(Deserialize, Validate)]
pub struct ConversationMessage {
    #[validate(length(min = 1))]
    pub prompt: String,
}
