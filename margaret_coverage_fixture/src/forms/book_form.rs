use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct BookForm {
    #[validate(length(min = 1))]
    pub title: String,
}
