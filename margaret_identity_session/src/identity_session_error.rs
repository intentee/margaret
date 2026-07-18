use thiserror::Error;

use margaret_cookie_jar::cookie_jar_error::CookieJarError;

#[derive(Debug, Error)]
pub enum IdentitySessionError {
    #[error("session cookie expiration timestamp {exp} is out of range: {source}")]
    CookieExpiration {
        exp: i64,
        #[source]
        source: cookie::time::error::ComponentRange,
    },

    #[error("a session cookie could not be staged in the cookie jar: {source}")]
    CookieStaging {
        #[source]
        source: CookieJarError,
    },
}
