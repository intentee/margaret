use serde::Serialize;

use crate::request_id::RequestId;

#[derive(Serialize)]
pub struct OutboundResponse<Payload> {
    pub id: RequestId,
    #[serde(rename = "final")]
    pub is_final: bool,
    pub method: String,
    #[serde(rename = "result")]
    pub payload: Payload,
}
