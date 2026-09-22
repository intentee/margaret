use serde::Serialize;

use crate::envelope_error::EnvelopeError;
use crate::request_id::RequestId;

#[derive(Serialize)]
#[serde(rename = "error", tag = "kind")]
pub struct OutboundError {
    pub error: EnvelopeError,
    pub id: RequestId,
}
