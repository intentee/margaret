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

impl ServerSentFrame {
    #[must_use]
    pub fn id(&self) -> &RequestId {
        match self {
            Self::Response { id, .. } | Self::Error { id, .. } => id,
        }
    }

    #[must_use]
    pub fn is_final(&self) -> bool {
        match self {
            Self::Response { is_done, .. } => *is_done,
            Self::Error { .. } => true,
        }
    }
}
