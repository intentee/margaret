use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::request_id::RequestId;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ClientSentFrame<Params = Value> {
    Request {
        id: RequestId,
        method: String,
        params: Params,
    },
    Notification {
        method: String,
        params: Params,
    },
}
