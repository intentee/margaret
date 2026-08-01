use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ConsumerGrant {
    pub permission: Vec<String>,
    pub resource_id: String,
}
