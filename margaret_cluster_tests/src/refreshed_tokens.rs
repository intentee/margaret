use serde::Deserialize;

#[derive(Deserialize)]
pub struct RefreshedTokens {
    pub access_token: String,
    pub refresh_token: String,
}
