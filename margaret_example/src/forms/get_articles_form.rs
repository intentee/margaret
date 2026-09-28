use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetArticlesForm {
    #[validate(length(min = 1))]
    pub author: Option<String>,
}
