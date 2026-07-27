use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouteOriginError {
    #[error("a route origin is not a valid URL: {source}")]
    InvalidUrl {
        #[from]
        source: url::ParseError,
    },

    #[error(
        "a route origin must be a canonical HTTPS origin without credentials, path, query, or fragment"
    )]
    NonCanonicalHttpsOrigin,
}
