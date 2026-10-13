use serde::Deserialize;
use validator::Validate;

use margaret::framework::macros::websocket_message;

#[websocket_message(notification, method = "whisper")]
#[derive(Deserialize, Validate)]
pub struct Whisper {
    #[validate(length(min = 1))]
    pub text: String,
}
