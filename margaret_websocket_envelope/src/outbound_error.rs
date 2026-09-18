use serde::Serialize;

use crate::envelope_error::EnvelopeError;
use crate::request_id::RequestId;

#[derive(Serialize)]
pub struct OutboundError {
    pub error: EnvelopeError,
    pub id: RequestId,
}
