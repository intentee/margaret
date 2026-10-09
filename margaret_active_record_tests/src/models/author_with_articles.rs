use margaret::framework::active_record::children::Children;
use margaret::framework::macros::eager_load;

use crate::models::article_with_translations::ArticleWithTranslations;
use crate::models::author::Author;

#[eager_load(model = Author)]
#[derive(Debug, PartialEq)]
pub struct AuthorWithArticles {
    #[base]
    pub author: Author,
    #[relation(articles, limit = 2)]
    pub articles: Children<ArticleWithTranslations>,
}
