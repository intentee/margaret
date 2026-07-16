#[derive(serde::Serialize)]
pub struct MintResponse {
    pub access_token: String,
    pub refresh_token: String,
}
