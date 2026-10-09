use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SessionRefreshRequest {
    pub secret: String,
}
