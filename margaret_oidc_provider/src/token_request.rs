use serde::Deserialize;
use validator::Validate;
use validator::ValidationErrors;

#[derive(Debug, Deserialize)]
#[serde(tag = "grant_type")]
pub enum TokenRequest {
    #[serde(rename = "authorization_code")]
    AuthorizationCode {
        client_id: Option<String>,
        code: String,
        code_verifier: String,
        redirect_uri: String,
        resource: Option<String>,
    },
    #[serde(rename = "client_credentials")]
    ClientCredentials {
        client_id: Option<String>,
        resource: Option<String>,
        scope: Option<String>,
    },
    #[serde(rename = "refresh_token")]
    RefreshToken {
        client_id: Option<String>,
        refresh_token: String,
        resource: Option<String>,
        scope: Option<String>,
    },
    #[serde(rename = "urn:ietf:params:oauth:grant-type:token-exchange")]
    TokenExchange {
        actor_token: Option<String>,
        audience: Option<String>,
        client_id: Option<String>,
        requested_token_type: Option<String>,
        resource: Option<String>,
        scope: Option<String>,
        subject_token: String,
        subject_token_type: String,
    },
    #[serde(other)]
    Unsupported,
}

impl TokenRequest {
    pub(crate) fn client_id(&self) -> Option<&str> {
        match self {
            Self::AuthorizationCode { client_id, .. }
            | Self::ClientCredentials { client_id, .. }
            | Self::RefreshToken { client_id, .. }
            | Self::TokenExchange { client_id, .. } => client_id.as_deref(),
            Self::Unsupported => None,
        }
    }
}

impl Validate for TokenRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}
