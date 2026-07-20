use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RequestId {
    Number(i64),
    Text(String),
}
