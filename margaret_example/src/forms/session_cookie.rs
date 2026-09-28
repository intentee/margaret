use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SessionCookie {
    pub session: Option<String>,
}
