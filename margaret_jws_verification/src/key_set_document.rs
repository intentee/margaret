use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub(crate) struct KeySetDocument {
    pub(crate) keys: Vec<Value>,
}
