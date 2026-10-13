use oauth2::ExtraTokenFields;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct IdTokenFields {
    pub(crate) id_token: String,
}

impl ExtraTokenFields for IdTokenFields {}
