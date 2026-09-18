use serde::Deserialize;
use serde_json::Value;

use crate::envelope_error::EnvelopeError;
use crate::request_id::RequestId;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ServerSentFrame {
    Response {
        id: RequestId,
        #[serde(rename = "done")]
        is_done: bool,
        method: String,
        #[serde(rename = "result")]
        payload: Value,
    },
    Error {
        error: EnvelopeError,
        id: RequestId,
    },
}
