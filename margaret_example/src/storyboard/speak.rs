use margaret_macros::websocket_message;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[websocket_message(method = "conversation.speak")]
pub struct Speak {
    #[validate(length(min = 1))]
    pub text: String,
}
