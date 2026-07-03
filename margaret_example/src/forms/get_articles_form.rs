#[derive(serde::Deserialize, validator::Validate)]
pub struct GetArticlesForm {
    #[validate(length(min = 1))]
    pub author: Option<String>,
}
