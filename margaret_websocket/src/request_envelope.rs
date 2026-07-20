use crate::outbound_response::OutboundResponse;
use crate::request_id::RequestId;

pub struct RequestEnvelope<Message> {
    id: RequestId,
    message: Message,
    method: String,
}

impl<Message> RequestEnvelope<Message> {
    #[must_use]
    pub fn new(id: RequestId, method: String, message: Message) -> Self {
        Self {
            id,
            message,
            method,
        }
    }

    #[must_use]
    pub fn message(&self) -> &Message {
        &self.message
    }

    #[must_use]
    pub fn response<Payload>(&self, payload: Payload) -> OutboundResponse<Payload> {
        OutboundResponse {
            id: self.id.clone(),
            is_done: true,
            method: self.method.clone(),
            payload,
        }
    }
}
