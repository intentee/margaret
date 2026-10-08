use thiserror::Error;

use margaret_database::database_error::DatabaseError;

#[derive(Debug, Error)]
pub enum AuthorizationGrantsDatabaseError {
    #[error("a refresh token cannot be found in the database: {0}")]
    FindRefreshToken(#[source] tokio_postgres::Error),

    #[error("an authorization grant cannot be serialized for the database: {0}")]
    GrantSerialization(#[source] serde_json::Error),

    #[error("a pending authorization cannot be held in the database: {0}")]
    HoldPendingAuthorization(#[source] tokio_postgres::Error),

    #[error("an authorization code cannot be issued in the database: {0}")]
    IssueCode(#[source] tokio_postgres::Error),

    #[error("a stored authorization grant is malformed: {0}")]
    MalformedGrant(#[source] serde_json::Error),

    #[error("a stored grant row is not of the declared shape: {0}")]
    MalformedRow(#[source] tokio_postgres::Error),

    #[error("the stored scopes of a refresh family are malformed: {0}")]
    MalformedScopes(#[source] serde_json::Error),

    #[error("a refresh family cannot be opened in the database: {0}")]
    OpenRefreshFamily(#[source] tokio_postgres::Error),

    #[error("an authorization code cannot be redeemed in the database: {0}")]
    RedeemCode(#[source] tokio_postgres::Error),

    #[error("a refresh family cannot be revoked in the database: {0}")]
    RevokeRefreshFamily(#[source] tokio_postgres::Error),

    #[error("a refresh token cannot be rotated in the database: {0}")]
    RotateRefreshToken(#[source] tokio_postgres::Error),

    #[error("the scopes of a refresh family cannot be serialized for the database: {0}")]
    ScopesSerialization(#[source] serde_json::Error),

    #[error("a pending authorization cannot be taken from the database: {0}")]
    TakePendingAuthorization(#[source] tokio_postgres::Error),

    #[error("the database holding the authorization grants is unavailable: {0}")]
    Unavailable(#[source] DatabaseError),
}
