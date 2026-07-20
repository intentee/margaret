use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeErrorCode {
    InvalidParams,
    UnknownMethod,
}
