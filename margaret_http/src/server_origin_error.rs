use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum ServerOriginError {
    #[error("the server url is malformed: {source}")]
    Malformed {
        #[source]
        source: url::ParseError,
    },
    #[error("the server url {url} carries more than its scheme, host and port")]
    NotAnOrigin { url: Url },
}
