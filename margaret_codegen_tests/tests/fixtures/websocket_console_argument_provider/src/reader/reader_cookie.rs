use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ReaderCookie {
    pub reader: Option<String>,
}
