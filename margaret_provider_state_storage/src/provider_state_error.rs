use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderStateError {
    #[error("the provider state could not decide a pending authorization: {source}")]
    DecidePendingAuthorization {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },

    #[error("the provider state could not hold a pending authorization: {source}")]
    HoldPendingAuthorization {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },

    #[error("the provider state could not issue an authorization code: {source}")]
    IssueCode {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },

    #[error("the provider state could not look up a presented authorization code: {source}")]
    PresentCode {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },

    #[error("the provider state could not look up a presented refresh token: {source}")]
    PresentRefreshToken {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },

    #[error("the provider state could not revoke a refresh token: {source}")]
    RevokeRefreshToken {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },

    #[error("the provider state could not rotate a refresh token: {source}")]
    RotateRefreshToken {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },

    #[error("the provider state could not spend a client assertion: {source}")]
    SpendClientAssertion {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },

    #[error("the provider state could not spend an authorization code: {source}")]
    SpendCode {
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
}
