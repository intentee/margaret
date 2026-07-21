use crate::outbound_response::OutboundResponse;
use crate::request_id::RequestId;
use crate::web_socket_response_message::WebSocketResponseMessage;

pub struct StreamingRequestEnvelope<Message> {
    id: RequestId,
    message: Message,
}

impl<Message> StreamingRequestEnvelope<Message> {
    #[must_use]
    pub fn new(id: RequestId, message: Message) -> Self {
        Self { id, message }
    }

    #[must_use]
    pub fn chunk<Payload>(&self, payload: Payload) -> OutboundResponse<Payload>
    where
        Payload: WebSocketResponseMessage,
    {
        OutboundResponse {
            id: self.id.clone(),
            is_done: false,
            method: Payload::METHOD,
            payload,
        }
    }

    #[must_use]
    pub fn fin<Payload>(&self, payload: Payload) -> OutboundResponse<Payload>
    where
        Payload: WebSocketResponseMessage,
    {
        OutboundResponse {
            id: self.id.clone(),
            is_done: true,
            method: Payload::METHOD,
            payload,
        }
    }

    #[must_use]
    pub fn message(&self) -> &Message {
        &self.message
    }
}
