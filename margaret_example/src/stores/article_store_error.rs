use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ArticleStoreError {
    #[error("no author exists with id '{author_id}'")]
    AuthorNotFound { author_id: Uuid },
}
