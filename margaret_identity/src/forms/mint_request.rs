#[derive(serde::Deserialize, validator::Validate)]
pub struct MintRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}
