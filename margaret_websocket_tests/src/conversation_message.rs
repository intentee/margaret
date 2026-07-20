use serde::Deserialize;
use validator::Validate;

use margaret_websocket::request_id::RequestId;
use margaret_websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret_websocket::web_socket_request_message::WebSocketRequestMessage;

#[derive(Deserialize, Validate)]
pub struct ConversationMessage {
    #[validate(length(min = 1))]
    pub prompt: String,
}

impl WebSocketRequestMessage for ConversationMessage {
    type Envelope = StreamingRequestEnvelope<Self>;

    fn envelope(id: RequestId, method: String, message: Self) -> Self::Envelope {
        StreamingRequestEnvelope::new(id, method, message)
    }
}
