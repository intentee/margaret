use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct PlanModule {
    #[serde(rename = "testModule")]
    pub test_module: String,
    pub variant: Value,
}
