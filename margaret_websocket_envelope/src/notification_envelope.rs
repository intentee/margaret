pub struct NotificationEnvelope<Message> {
    message: Message,
}

impl<Message> NotificationEnvelope<Message> {
    #[must_use]
    pub fn new(message: Message) -> Self {
        Self { message }
    }

    #[must_use]
    pub fn message(&self) -> &Message {
        &self.message
    }
}
