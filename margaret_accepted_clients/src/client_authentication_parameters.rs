use serde::Deserialize;

use margaret_oauth_vocabulary::optional_parameter::optional_parameter;

#[derive(Debug, Default, Deserialize)]
pub struct ClientAuthenticationParameters {
    #[serde(default, deserialize_with = "optional_parameter")]
    pub client_assertion: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub client_assertion_type: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub client_id: Option<String>,
}
