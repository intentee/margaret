use thiserror::Error;

use crate::max_identifier_bytes::MAX_IDENTIFIER_BYTES;

#[derive(Debug, Error)]
pub enum SchemaIdentifierNamingError {
    #[error(
        "schema identifier '{identifier}' is {length} bytes, which exceeds the PostgreSQL limit of {} bytes",
        MAX_IDENTIFIER_BYTES
    )]
    IdentifierTooLong { identifier: String, length: usize },
}
