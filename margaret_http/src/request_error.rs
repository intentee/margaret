use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
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
}
