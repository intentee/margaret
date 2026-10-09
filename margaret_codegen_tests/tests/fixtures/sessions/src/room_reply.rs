use serde::Serialize;

use margaret::framework::macros::websocket_message;

#[websocket_message(response, method = "room_reply")]
#[derive(Serialize)]
pub struct RoomReply {
    pub text: String,
}
