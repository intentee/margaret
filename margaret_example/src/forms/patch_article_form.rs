use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct PatchArticleForm {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub body: Option<String>,
}
