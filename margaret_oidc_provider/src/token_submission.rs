use serde::Deserialize;
use validator::Validate;
use validator::ValidationErrors;

use margaret_accepted_clients::client_authentication_parameters::ClientAuthenticationParameters;
use margaret_oauth_vocabulary::required_parameter::required_parameter;

#[derive(Debug, Deserialize)]
pub struct TokenSubmission {
    #[serde(flatten)]
    pub client_authentication: ClientAuthenticationParameters,
    #[serde(deserialize_with = "required_parameter")]
    pub token: String,
}

impl Validate for TokenSubmission {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}
