use margaret::framework::macros::websocket_message;
use serde::Deserialize;
use validator::Validate;

#[websocket_message(request, method = "conversation_message", response = stream)]
#[derive(Deserialize, Validate)]
pub struct ConversationMessage {
    #[validate(length(min = 1))]
    pub prompt: String,
}
