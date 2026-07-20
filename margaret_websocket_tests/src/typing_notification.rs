use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct TypingNotification {
    #[validate(length(min = 1))]
    pub who: String,
}
