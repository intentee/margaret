use margaret::framework::macros::eager_load;

use crate::models::article::Article;
use crate::models::author_with_profile::AuthorWithProfile;

#[eager_load(model = Article)]
#[derive(Debug, PartialEq)]
pub struct ArticleWithAuthorProfile {
    #[base]
    pub article: Article,
    #[relation(author)]
    pub author: AuthorWithProfile,
}
