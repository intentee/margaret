use crate::request_id::RequestId;

pub struct Envelope<Message> {
    pub id: RequestId,
    pub message: Message,
}

impl<Message> Envelope<Message> {
    #[must_use]
    pub fn new(id: RequestId, message: Message) -> Self {
        Self { id, message }
    }
}
