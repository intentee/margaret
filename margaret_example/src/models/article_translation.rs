use margaret::framework::active_record::key::Key;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::models::article::Article;

#[model(table = "article_translations")]
#[primary_key(fields = [article, locale])]
#[derive(Clone)]
pub struct ArticleTranslation {
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    pub article: Key<Article>,
    #[column]
    pub locale: String,
    #[column]
    pub title: String,
}
