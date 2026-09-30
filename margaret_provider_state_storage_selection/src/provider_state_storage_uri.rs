use std::str::FromStr;

use sqlx::postgres::PgConnectOptions;
use url::Url;

use crate::provider_state_storage_uri_error::ProviderStateStorageUriError;

const MEMORY_STORAGE: &str = "memory";
const POSTGRES_SCHEMES: [&str; 2] = ["postgres", "postgresql"];

#[derive(Clone, Debug)]
pub enum ProviderStateStorageUri {
    Memory,
    Postgres(Box<PgConnectOptions>),
}

impl FromStr for ProviderStateStorageUri {
    type Err = ProviderStateStorageUriError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let unknown = || ProviderStateStorageUriError::UnknownStorage {
            uri: value.to_string(),
        };

        if value == MEMORY_STORAGE {
            return Ok(Self::Memory);
        }

        match Url::parse(value) {
            Ok(url) if POSTGRES_SCHEMES.contains(&url.scheme()) => value
                .parse()
                .map(|options| Self::Postgres(Box::new(options)))
                .map_err(|source| ProviderStateStorageUriError::PostgresOptions {
                    uri: value.to_string(),
                    source,
                }),
            Ok(_) | Err(_) => Err(unknown()),
        }
    }
}
