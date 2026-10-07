use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpsUrlError {
    #[error("the url is malformed: {source}")]
    Malformed {
        #[source]
        source: url::ParseError,
    },

    #[error("the url uses the '{scheme}' scheme instead of https")]
    NotHttps { scheme: String },
}
