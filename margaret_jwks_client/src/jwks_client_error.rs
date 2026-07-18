use thiserror::Error;
use url::ParseError;

use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum JwksClientError {
    #[error("the jwks document could not be fetched from the issuer: {0}")]
    DocumentFetch(#[source] reqwest::Error),

    #[error("the jwks http client could not be built: {0}")]
    HttpClientBuild(#[source] reqwest::Error),

    #[error("the issuer url '{issuer_url}' cannot carry the well known jwks path: {source}")]
    IssuerUrlNotABase {
        issuer_url: String,
        #[source]
        source: ParseError,
    },

    #[error("the jwks document has not been fetched from the issuer yet")]
    NotReady,

    #[error("the token expired before it was verified")]
    TokenExpired,

    #[error("the token could not be verified against the published jwks: {0}")]
    TokenVerification(#[source] JwksKeyError),
}
