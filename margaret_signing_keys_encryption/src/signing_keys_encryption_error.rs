use aws_lc_rs::error::Unspecified;
use thiserror::Error;

#[derive(Debug, Eq, Error, PartialEq)]
pub enum SigningKeysEncryptionError {
    #[error(
        "the signing keys encryption key is {length} bytes long where AES-256-GCM takes {expected}"
    )]
    KeyLength { expected: usize, length: usize },

    #[error("the signing keys encryption key is not standard padded base64: {source}")]
    KeyNotBase64 {
        #[source]
        source: base64ct::Error,
    },

    #[error("AES-256-GCM rejected the signing keys encryption key: {0}")]
    KeyRejected(#[source] Unspecified),

    #[error(
        "the sealed signing keys do not open under the encryption key, so the key is another one or the stored document was altered: {0}"
    )]
    Open(#[source] Unspecified),

    #[error("the signing keys could not be sealed: {0}")]
    Seal(#[source] Unspecified),
}
