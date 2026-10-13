use oauth2::ExtraTokenFields;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub(crate) struct ExtensionMembers {
    pub(crate) members: Map<String, Value>,
}

impl ExtraTokenFields for ExtensionMembers {}
