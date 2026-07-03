#[derive(serde::Deserialize, validator::Validate)]
pub struct PatchArticleForm {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub body: Option<String>,
}
