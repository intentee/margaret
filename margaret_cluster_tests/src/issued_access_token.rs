use serde::Deserialize;

#[derive(Deserialize)]
pub struct IssuedAccessToken {
    pub access_token: String,
}
