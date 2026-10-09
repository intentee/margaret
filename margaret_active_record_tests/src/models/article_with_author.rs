use margaret::framework::macros::eager_load;

use crate::models::article::Article;
use crate::models::author::Author;

#[eager_load(model = Article)]
#[derive(Debug, PartialEq)]
pub struct ArticleWithAuthor {
    #[base]
    pub article: Article,
    #[relation(author)]
    pub author: Author,
}
