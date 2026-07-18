use http::header::ToStrError;
use thiserror::Error;

use margaret_cookie_jar::cookie_jar_error::CookieJarError;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("the request cookies could not be read: {source}")]
    CookieJar {
        #[source]
        source: CookieJarError,
    },

    #[error("a request header value is not visible ASCII text: {source}")]
    HeaderNotText {
        #[from]
        source: ToStrError,
    },

    #[error("the request content type is not a valid media type: {source}")]
    MalformedContentType {
        #[source]
        source: mime::FromStrError,
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
