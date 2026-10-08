use serde::Serialize;

#[derive(Serialize)]
pub struct SessionRefreshToken {
    pub refresh_token: String,
}
