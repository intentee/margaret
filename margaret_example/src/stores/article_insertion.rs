use crate::models::article::Article;
use crate::stores::author_not_found::AuthorNotFound;

pub enum ArticleInsertion {
    AuthorNotFound(AuthorNotFound),
    Inserted(Article),
}
