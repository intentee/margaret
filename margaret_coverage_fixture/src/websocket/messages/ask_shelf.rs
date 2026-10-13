use serde::Deserialize;
use validator::Validate;

use margaret::framework::macros::websocket_message;
use margaret::framework::websocket::web_socket_response::WebSocketResponse;

#[websocket_message(request, method = "ask_shelf", response = WebSocketResponse::Single)]
#[derive(Deserialize, Validate)]
pub struct AskShelf {
    #[validate(length(min = 1))]
    pub question: String,
}
