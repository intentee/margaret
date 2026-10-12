use serde::Deserialize;
use validator::Validate;

use margaret::framework::macros::websocket_message;

#[websocket_message(notification, method = "typing")]
#[derive(Deserialize, Validate)]
pub struct Typing {
    #[validate(length(min = 1))]
    pub who: String,
}
