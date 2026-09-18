use crate::request_id::RequestId;

pub trait WebSocketRequestMessage: Sized {
    const METHOD: &'static str;

    type Envelope;

    fn envelope(id: RequestId, message: Self) -> Self::Envelope;
}
