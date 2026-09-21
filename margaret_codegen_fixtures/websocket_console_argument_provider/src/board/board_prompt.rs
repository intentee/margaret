use serde::Deserialize;
use validator::Validate;

use margaret::framework::macros::websocket_message;

#[websocket_message(request, method = "board_prompt", response = stream)]
#[derive(Deserialize, Validate)]
pub struct BoardPrompt {
    #[validate(length(min = 1))]
    pub prompt: String,
}
