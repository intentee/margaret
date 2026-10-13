use bytes::Bytes;

use crate::body_rejection::BodyRejection;

#[derive(Debug)]
pub enum RequestBodyChunk {
    Data(Bytes),
    End,
    Rejected(BodyRejection),
}
