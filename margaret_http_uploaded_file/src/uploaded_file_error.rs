use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UploadedFileError {
    #[error("an uploaded file temporary file could not be created: {source}")]
    UploadTempFile {
        #[source]
        source: io::Error,
    },

    #[error("an uploaded file could not be written to its temporary file: {source}")]
    UploadWrite {
        #[source]
        source: io::Error,
    },
}
