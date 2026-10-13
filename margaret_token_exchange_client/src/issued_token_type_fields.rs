use oauth2::ExtraTokenFields;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct IssuedTokenTypeFields {
    pub issued_token_type: Option<String>,
}

impl ExtraTokenFields for IssuedTokenTypeFields {}
