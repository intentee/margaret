use serde::Deserialize;

#[derive(Deserialize)]
pub struct IssuedTokens {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: String,
}
