use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwksSecretStorageUriError {
    #[error(
        "the 'file' jwks secret storage uri requires a path, e.g. 'file:/var/lib/app/jwks.json'"
    )]
    FileRequiresPath,

    #[error("the 'memory' jwks secret storage uri does not take a path, but '{path}' was given")]
    MemoryTakesNoPath { path: String },

    #[error("unknown jwks secret storage scheme '{scheme}'; expected 'memory' or 'file:<path>'")]
    UnknownScheme { scheme: String },
}
