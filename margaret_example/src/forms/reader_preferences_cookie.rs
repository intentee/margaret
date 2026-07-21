#[derive(serde::Deserialize, validator::Validate)]
pub struct ReaderPreferencesCookie {
    #[validate(length(min = 1))]
    pub theme: Option<String>,
}
