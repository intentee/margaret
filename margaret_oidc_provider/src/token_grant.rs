use serde::Deserialize;

use margaret_oauth_vocabulary::optional_parameter::optional_parameter;
use margaret_oauth_vocabulary::required_parameter::required_parameter;

#[derive(Debug, Deserialize)]
#[serde(tag = "grant_type")]
pub enum TokenGrant {
    #[serde(rename = "authorization_code")]
    AuthorizationCode {
        #[serde(deserialize_with = "required_parameter")]
        code: String,
        #[serde(deserialize_with = "required_parameter")]
        code_verifier: String,
        #[serde(deserialize_with = "required_parameter")]
        redirect_uri: String,
        #[serde(default, deserialize_with = "optional_parameter")]
        resource: Option<String>,
    },
    #[serde(rename = "client_credentials")]
    ClientCredentials {
        #[serde(default, deserialize_with = "optional_parameter")]
        resource: Option<String>,
        #[serde(default, deserialize_with = "optional_parameter")]
        scope: Option<String>,
    },
    #[serde(rename = "")]
    GrantTypeOmitted,
    #[serde(rename = "refresh_token")]
    RefreshToken {
        #[serde(deserialize_with = "required_parameter")]
        refresh_token: String,
        #[serde(default, deserialize_with = "optional_parameter")]
        resource: Option<String>,
        #[serde(default, deserialize_with = "optional_parameter")]
        scope: Option<String>,
    },
    #[serde(rename = "urn:ietf:params:oauth:grant-type:token-exchange")]
    TokenExchange {
        #[serde(default, deserialize_with = "optional_parameter")]
        actor_token: Option<String>,
        #[serde(default, deserialize_with = "optional_parameter")]
        audience: Option<String>,
        #[serde(default, deserialize_with = "optional_parameter")]
        requested_token_type: Option<String>,
        #[serde(default, deserialize_with = "optional_parameter")]
        resource: Option<String>,
        #[serde(default, deserialize_with = "optional_parameter")]
        scope: Option<String>,
        #[serde(deserialize_with = "required_parameter")]
        subject_token: String,
        #[serde(deserialize_with = "required_parameter")]
        subject_token_type: String,
    },
    #[serde(other)]
    Unsupported,
}
