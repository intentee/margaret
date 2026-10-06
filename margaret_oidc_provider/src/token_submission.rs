use serde::Deserialize;
use validator::Validate;
use validator::ValidationErrors;

use margaret_oauth_vocabulary::optional_parameter::optional_parameter;
use margaret_oauth_vocabulary::required_parameter::required_parameter;

#[derive(Debug, Deserialize)]
pub struct TokenSubmission {
    #[serde(default, deserialize_with = "optional_parameter")]
    pub client_id: Option<String>,
    #[serde(deserialize_with = "required_parameter")]
    pub token: String,
}

impl Validate for TokenSubmission {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}
