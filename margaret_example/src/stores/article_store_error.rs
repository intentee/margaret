use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ArticleStoreError {
    #[error("no author exists with id '{author_id}'")]
    AuthorNotFound { author_id: Uuid },

    #[error("an article store query failed: {source}")]
    Query {
        #[from]
        source: sqlx::Error,
    },

    #[error("the stored article status '{value}' is not recognized")]
    UnknownStatus { value: String },
}
