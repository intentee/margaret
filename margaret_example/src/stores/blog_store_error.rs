use thiserror::Error;

use margaret::framework::database::database_error::DatabaseError;
use margaret::framework::tokio_postgres;

#[derive(Debug, Error)]
pub enum BlogStoreError {
    #[error("failed to find the article: {0}")]
    FindArticle(#[source] tokio_postgres::Error),

    #[error("failed to find the user of a session: {0}")]
    FindSessionUser(#[source] tokio_postgres::Error),

    #[error("failed to find the name of a user: {0}")]
    FindUserName(#[source] tokio_postgres::Error),

    #[error("failed to insert the article: {0}")]
    InsertArticle(#[source] tokio_postgres::Error),

    #[error("failed to list the articles: {0}")]
    ListArticles(#[source] tokio_postgres::Error),

    #[error("an article row does not match the article model: {0}")]
    MalformedArticleRow(#[source] tokio_postgres::Error),

    #[error("a session row does not match the user model: {0}")]
    MalformedSessionRow(#[source] tokio_postgres::Error),

    #[error("a user row does not match the user model: {0}")]
    MalformedUserRow(#[source] tokio_postgres::Error),

    #[error("failed to remove the article: {0}")]
    RemoveArticle(#[source] tokio_postgres::Error),

    #[error("failed to save the article: {0}")]
    SaveArticle(#[source] tokio_postgres::Error),

    #[error("failed to seed the blog: {0}")]
    Seed(#[source] tokio_postgres::Error),

    #[error("failed to start a session: {0}")]
    StartSession(#[source] tokio_postgres::Error),

    #[error("the stored article status '{status}' is not an article status")]
    UnknownArticleStatus { status: String },

    #[error("the blog database is unavailable: {0}")]
    Unavailable(#[source] DatabaseError),
}
