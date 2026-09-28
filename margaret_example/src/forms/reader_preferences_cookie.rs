use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ReaderPreferencesCookie {
    #[validate(length(min = 1))]
    pub theme: Option<String>,
}
