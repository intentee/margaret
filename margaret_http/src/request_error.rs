use cookie::ParseError;
use http::header::ToStrError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("a request header value is not visible ASCII text: {source}")]
    HeaderNotText {
        #[from]
        source: ToStrError,
    },

    #[error("a request cookie could not be parsed: {source}")]
    MalformedCookie {
        #[from]
        source: ParseError,
    },
}
