#[derive(serde::Deserialize, validator::Validate)]
pub struct SessionCookie {
    pub session: Option<String>,
}
