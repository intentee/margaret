use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct MintAccessTokenRequest {
    pub refresh_token: String,
}
