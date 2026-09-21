use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

use margaret_websocket_envelope::envelopes_web_socket_request::EnvelopesWebSocketRequest;
use margaret_websocket_envelope::request_envelope::RequestEnvelope;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::web_socket_request_message::WebSocketRequestMessage;

#[derive(Deserialize, Serialize, Validate)]
pub struct SilentMessage {
    #[validate(length(min = 1))]
    pub label: String,
}

impl WebSocketRequestMessage for SilentMessage {
    const METHOD: &'static str = "silent";
}

impl EnvelopesWebSocketRequest for SilentMessage {
    type Envelope = RequestEnvelope<Self>;

    fn envelope(id: RequestId, message: Self) -> Self::Envelope {
        RequestEnvelope::new(id, message)
    }
}
