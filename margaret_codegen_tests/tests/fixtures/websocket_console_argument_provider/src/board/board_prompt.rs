use serde::Deserialize;
use validator::Validate;

use margaret::framework::macros::websocket_message;
use margaret::framework::websocket::web_socket_response::WebSocketResponse;

#[websocket_message(request, method = "board_prompt", response = WebSocketResponse::Stream)]
#[derive(Deserialize, Validate)]
pub struct BoardPrompt {
    #[validate(length(min = 1))]
    pub prompt: String,
}
