use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("an HTTP route pattern is invalid: {source}")]
    InvalidPattern {
        #[from]
        source: matchit::InsertError,
    },

    #[error("an HTTP method is invalid: {source}")]
    InvalidMethod {
        #[from]
        source: http::method::InvalidMethod,
    },

    #[error("an HTTP header value is invalid: {source}")]
    InvalidHeaderValue {
        #[from]
        source: http::header::InvalidHeaderValue,
    },

    #[error("route '{path}' registers method '{method}' more than once")]
    DuplicateMethod {
        method: http::Method,
        path: &'static str,
    },
}
