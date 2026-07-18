use http::header::ToStrError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CookieJarError {
    #[error("the cookie `{name}` is already removed in this request")]
    AlreadyRemoved { name: String },

    #[error("the cookie `{name}` is already set in this request")]
    AlreadySet { name: String },

    #[error("the cookie domain is empty")]
    DomainEmpty,

    #[error("the cookie domain `{value}` is an IP address, which cannot scope a cookie")]
    DomainIsIpAddress { value: String },

    #[error("the cookie domain contains an empty label")]
    DomainLabelEmpty,

    #[error("the cookie domain label `{label}` starts or ends with a hyphen")]
    DomainLabelHyphenBoundary { label: String },

    #[error("the cookie domain label `{label}` contains the invalid character `{character}`")]
    DomainLabelInvalidCharacter { character: char, label: String },

    #[error("the cookie domain label `{label}` is longer than 63 characters")]
    DomainLabelTooLong { label: String },

    #[error("the cookie domain `{value}` starts with a dot")]
    DomainStartsWithDot { value: String },

    #[error("the cookie domain is {length} characters long, which exceeds the limit of 255")]
    DomainTooLong { length: usize },

    #[error("the request carries the cookie `{name}` more than once")]
    DuplicateInRequest { name: String },

    #[error("the cookie `{name}` requires a secure connection to use `SameSite=None`")]
    InsecureSameSiteNone { name: String },

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
