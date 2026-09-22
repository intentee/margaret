use serde::Serialize;

use crate::request_id::RequestId;

#[derive(Serialize)]
#[serde(rename = "response", tag = "kind")]
pub struct OutboundResponse<Payload> {
    pub id: RequestId,
    #[serde(rename = "done")]
    pub is_done: bool,
    pub method: &'static str,
    #[serde(rename = "result")]
    pub payload: Payload,
}
