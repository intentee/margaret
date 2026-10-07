use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageUriError {
    #[error(
        "the 'file' storage uri must name an absolute local path, e.g. 'file:///var/lib/app/jwks.json'"
    )]
    FileNotAbsolutePath,

    #[error("the storage uri is not a url: {source}")]
    Malformed {
        #[source]
        source: url::ParseError,
    },

    #[error("the 'memory' storage uri takes no location; write it as 'memory:'")]
    MemoryWithLocation,

    #[error(
        "the storage uri scheme '{scheme}' is not 'memory', 'file', 'postgres' or 'postgresql'"
    )]
    UnknownScheme { scheme: String },
}
