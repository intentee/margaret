use margaret::framework::macros::model;

#[model(table = "article_translations")]
#[primary_key(columns = [article_id, locale])]
#[foreign_key(
    columns = [article_id],
    references = crate::models::article::Article,
    on_delete = cascade
)]
#[derive(Clone)]
pub struct ArticleTranslation {
    #[column]
    pub article_id: uuid::Uuid,
    #[column]
    pub locale: String,
    #[column]
    pub title: String,
}
