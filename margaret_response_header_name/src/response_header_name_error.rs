use http::header::InvalidHeaderName;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResponseHeaderNameError {
    #[error(
        "the `set-cookie` response header is produced by the cookie jar and cannot be set directly"
    )]
    CookieJarOwnedName,

    #[error("`{name}` is not a valid response header name: {source}")]
    MalformedName {
        name: String,
        #[source]
        source: InvalidHeaderName,
    },
}
