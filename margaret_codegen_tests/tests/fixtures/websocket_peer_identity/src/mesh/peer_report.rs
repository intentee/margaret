use serde::Serialize;

use margaret::framework::macros::websocket_message;

#[websocket_message(response, method = "peer_report")]
#[derive(Serialize)]
pub struct PeerReport {
    pub spiffe_id: String,
}
