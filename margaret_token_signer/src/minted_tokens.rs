use serde::Serialize;

#[derive(Serialize)]
pub struct MintedTokens {
    pub access_token: String,
    pub refresh_token: String,
}
