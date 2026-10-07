use std::path::PathBuf;
use std::str::FromStr;

use margaret_storage_uri::storage_uri::StorageUri;

use crate::jwks_secret_storage_uri_error::JwksSecretStorageUriError;

#[derive(Clone, Debug, PartialEq)]
pub enum JwksSecretStorageUri {
    File { path: PathBuf },
    Memory,
}

impl FromStr for JwksSecretStorageUri {
    type Err = JwksSecretStorageUriError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.parse::<StorageUri>()? {
            StorageUri::File { path } => Ok(Self::File { path }),
            StorageUri::Memory => Ok(Self::Memory),
            StorageUri::Postgres { .. } => Err(JwksSecretStorageUriError::PostgresUnsupported),
        }
    }
}
