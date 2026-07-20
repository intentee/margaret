use serde::Deserialize;
use validator::Validate;

use margaret_websocket::request_envelope::RequestEnvelope;
use margaret_websocket::request_id::RequestId;
use margaret_websocket::web_socket_request_message::WebSocketRequestMessage;

#[derive(Deserialize, Validate)]
pub struct PingMessage {
    #[validate(length(min = 1))]
    pub label: String,
}

impl WebSocketRequestMessage for PingMessage {
    type Envelope = RequestEnvelope<Self>;

    fn envelope(id: RequestId, method: String, message: Self) -> Self::Envelope {
        RequestEnvelope::new(id, method, message)
    }
}
