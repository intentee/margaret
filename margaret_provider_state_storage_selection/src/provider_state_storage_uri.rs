use std::str::FromStr;

use sqlx::ConnectOptions;
use sqlx::postgres::PgConnectOptions;

use margaret_storage_uri::storage_uri::StorageUri;

use crate::provider_state_storage_uri_error::ProviderStateStorageUriError;

#[derive(Clone)]
pub enum ProviderStateStorageUri {
    Memory,
    Postgres(Box<PgConnectOptions>),
}

impl FromStr for ProviderStateStorageUri {
    type Err = ProviderStateStorageUriError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.parse::<StorageUri>()? {
            StorageUri::File { .. } => Err(ProviderStateStorageUriError::FileUnsupported),
            StorageUri::Memory => Ok(Self::Memory),
            StorageUri::Postgres { url } => PgConnectOptions::from_url(&url)
                .map(|options| Self::Postgres(Box::new(options)))
                .map_err(|source| ProviderStateStorageUriError::PostgresOptions { source }),
        }
    }
}
