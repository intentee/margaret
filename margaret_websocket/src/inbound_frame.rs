use serde::Deserialize;
use serde_json::Value;

use crate::request_id::RequestId;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum InboundFrame {
    Request {
        id: RequestId,
        method: String,
        params: Value,
    },
    Notification {
        method: String,
        params: Value,
    },
}
