use http::header::ToStrError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CookieJarError {
    #[error("the cookie `{name}` is already removed in this request")]
    AlreadyRemoved { name: String },

    #[error("the cookie `{name}` is already set in this request")]
    AlreadySet { name: String },

    #[error("the request carries the cookie `{name}` more than once")]
    DuplicateInRequest { name: String },

    #[error("a cookie in the request header could not be parsed: {source}")]
    MalformedCookie {
        #[source]
        source: cookie::ParseError,
    },

    #[error("the cookie `{name}` is not in the request")]
    NotInRequest { name: String },

    #[error("the request cookie header is not visible ASCII text: {source}")]
    UnreadableHeader {
        #[source]
        source: ToStrError,
    },
}
