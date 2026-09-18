use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RequestId {
    Number(i64),
    Text(String),
}
