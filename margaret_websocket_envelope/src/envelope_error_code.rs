use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeErrorCode {
    InternalError,
    InvalidParams,
    UnknownMethod,
}
