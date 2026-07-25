use margaret::framework::http::response::Response;

use crate::stores::article_store_error::ArticleStoreError;

#[must_use]
pub fn respond_to_article_store_error(error: &ArticleStoreError) -> Response {
    match error {
        ArticleStoreError::AuthorNotFound { author_id } => {
            Response::text(500, format!("no author exists with id '{author_id}'"))
        }
        ArticleStoreError::Query { .. } | ArticleStoreError::UnknownStatus { .. } => {
            eprintln!("margaret_example: an article store operation failed: {error}");

            Response::text(500, "Internal Server Error")
        }
    }
}
