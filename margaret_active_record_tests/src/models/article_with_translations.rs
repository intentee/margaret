use margaret::framework::active_record::children::Children;
use margaret::framework::macros::eager_load;

use crate::models::article::Article;
use crate::models::article_translation::ArticleTranslation;

#[eager_load(model = Article)]
#[derive(Debug, PartialEq)]
pub struct ArticleWithTranslations {
    #[base]
    pub article: Article,
    #[relation(translations, limit = 2)]
    pub translations: Children<ArticleTranslation>,
}
