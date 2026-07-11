use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentitySessionError {
    #[error("session cookie expiration timestamp {exp} is out of range: {source}")]
    CookieExpiration {
        exp: i64,
        #[source]
        source: cookie::time::error::ComponentRange,
    },
}
