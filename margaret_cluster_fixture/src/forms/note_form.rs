use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NoteForm {
    #[validate(length(min = 1))]
    pub body: String,
    pub expires_at: Option<DateTime<Utc>>,
}
