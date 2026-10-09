use serde::Deserialize;
use validator::Validate;

use margaret::framework::macros::websocket_message;
use margaret::framework::websocket::web_socket_response::WebSocketResponse;

#[websocket_message(request, method = "who_am_i", response = WebSocketResponse::Single)]
#[derive(Deserialize, Validate)]
pub struct WhoAmI {}
