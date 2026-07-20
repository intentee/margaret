use crate::outbound_response::OutboundResponse;
use crate::request_id::RequestId;

pub struct StreamingRequestEnvelope<Message> {
    id: RequestId,
    message: Message,
    method: String,
}

impl<Message> StreamingRequestEnvelope<Message> {
    #[must_use]
    pub fn new(id: RequestId, method: String, message: Message) -> Self {
        Self {
            id,
            message,
            method,
        }
    }

    #[must_use]
    pub fn chunk<Payload>(&self, payload: Payload) -> OutboundResponse<Payload> {
        OutboundResponse {
            id: self.id.clone(),
            is_done: false,
            method: self.method.clone(),
            payload,
        }
    }

    #[must_use]
    pub fn fin<Payload>(&self, payload: Payload) -> OutboundResponse<Payload> {
        OutboundResponse {
            id: self.id.clone(),
            is_done: true,
            method: self.method.clone(),
            payload,
        }
    }

    #[must_use]
    pub fn message(&self) -> &Message {
        &self.message
    }
}
