use crate::request_id::RequestId;

pub trait EnvelopesWebSocketRequest: Sized {
    type Envelope;

    fn envelope(id: RequestId, message: Self) -> Self::Envelope;
}
