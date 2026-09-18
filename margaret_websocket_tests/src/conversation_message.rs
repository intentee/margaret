use serde::Deserialize;
use validator::Validate;

use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::streaming_request_envelope::StreamingRequestEnvelope;
use margaret_websocket_envelope::web_socket_request_message::WebSocketRequestMessage;

#[derive(Deserialize, Validate)]
pub struct ConversationMessage {
    #[validate(length(min = 1))]
    pub prompt: String,
}

impl WebSocketRequestMessage for ConversationMessage {
    const METHOD: &'static str = "conversation_message";

    type Envelope = StreamingRequestEnvelope<Self>;

    fn envelope(id: RequestId, message: Self) -> Self::Envelope {
        StreamingRequestEnvelope::new(id, message)
    }
}
