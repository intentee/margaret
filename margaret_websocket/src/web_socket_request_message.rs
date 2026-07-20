use crate::request_id::RequestId;

pub trait WebSocketRequestMessage: Sized {
    type Envelope;

    fn envelope(id: RequestId, method: String, message: Self) -> Self::Envelope;
}
