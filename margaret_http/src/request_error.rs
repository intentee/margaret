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

    #[error("the request body is not valid JSON: {source}")]
    MalformedJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("the multipart request body could not be parsed: {source}")]
    Multipart {
        #[source]
        source: multer::Error,
    },

    #[error("the multipart request is missing its Content-Type boundary")]
    MissingMultipartBoundary,

    #[error("the request body exceeds the {limit} byte upload limit")]
    PayloadTooLarge { limit: u64 },

    #[error("the server received an uploaded file but file uploads are disabled")]
    UploadsDisabled,

    #[error("an uploaded file temporary file could not be created: {source}")]
    UploadTempFile {
        #[source]
        source: std::io::Error,
    },

    #[error("an uploaded file could not be written to its temporary file: {source}")]
    UploadWrite {
        #[source]
        source: std::io::Error,
    },

    #[error("the request body could not be read: {source}")]
    BodyRead {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
