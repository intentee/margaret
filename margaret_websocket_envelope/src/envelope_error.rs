use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::envelope_error_code::EnvelopeErrorCode;

#[derive(Deserialize, Serialize)]
pub struct EnvelopeError {
    pub code: EnvelopeErrorCode,
    pub details: Value,
    pub message: String,
}
