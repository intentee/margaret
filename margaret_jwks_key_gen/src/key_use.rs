use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum KeyUse {
    #[serde(rename = "sig")]
    Signature,
}
