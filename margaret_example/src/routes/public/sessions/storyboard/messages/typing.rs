use margaret::framework::macros::websocket_message;
use serde::Deserialize;
use validator::Validate;

#[websocket_message(notification, method = "typing")]
#[derive(Deserialize, Validate)]
pub struct Typing {
    #[validate(length(min = 1))]
    pub who: String,
}
