use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct SessionRefreshToken {
    pub refresh_token: String,
}
