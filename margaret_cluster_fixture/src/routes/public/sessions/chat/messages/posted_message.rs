use serde::Serialize;
use uuid::Uuid;

use margaret::framework::macros::websocket_message;

#[websocket_message(response, method = "posted_message")]
#[derive(Serialize)]
pub struct PostedMessage {
    pub id: Uuid,
}
