use margaret::framework::macros::model;

#[model(table = "article_translations")]
#[foreign_key(
    columns = [article_id],
    references = crate::models::article::Article,
    on_delete = cascade
)]
#[derive(Clone)]
pub struct ArticleTranslation {
    #[column(primary_key)]
    pub article_id: uuid::Uuid,
    #[column(primary_key)]
    pub locale: String,
    #[column]
    pub title: String,
}
