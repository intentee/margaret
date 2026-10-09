use oauth2::ExtraTokenFields;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ProviderTokenFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) issued_token_type: Option<String>,
}

impl ExtraTokenFields for ProviderTokenFields {}
