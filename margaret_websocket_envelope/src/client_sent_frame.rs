use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::request_id::RequestId;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ClientSentFrame {
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
