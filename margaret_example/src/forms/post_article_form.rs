#[derive(serde::Deserialize, validator::Validate)]
pub struct PostArticleForm {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub body: String,
    #[validate(length(min = 1))]
    pub author_id: String,
}
