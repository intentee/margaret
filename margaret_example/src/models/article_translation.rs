use uuid::Uuid;

use margaret::framework::macros::model;

use crate::models::article::Article;

#[model(table = "article_translations")]
#[primary_key(columns = [article_id, locale])]
#[foreign_key(
    columns = [article_id],
    references = Article,
    on_delete = cascade
)]
#[derive(Clone)]
pub struct ArticleTranslation {
    #[column]
    pub article_id: Uuid,
    #[column]
    pub locale: String,
    #[column]
    pub title: String,
}
