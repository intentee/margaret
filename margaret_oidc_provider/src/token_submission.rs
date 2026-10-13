use serde::Deserialize;
use validator::Validate;

use margaret_accepted_clients::client_authentication_parameters::ClientAuthenticationParameters;
use margaret_oauth_vocabulary::required_parameter::required_parameter;

#[derive(Debug, Deserialize, Validate)]
pub struct TokenSubmission {
    #[serde(flatten)]
    pub client_authentication: ClientAuthenticationParameters,
    #[serde(deserialize_with = "required_parameter")]
    pub token: String,
}
