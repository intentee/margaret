use serde::Serialize;

use margaret::framework::macros::websocket_message;

#[websocket_message(response, method = "identity")]
#[derive(Serialize)]
pub struct Identity {
    pub spiffe_id: String,
}
