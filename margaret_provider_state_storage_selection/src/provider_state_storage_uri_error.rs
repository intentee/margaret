use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderStateStorageUriError {
    #[error(
        "the openid connect provider state storage '{uri}' is not valid postgres options: {source}"
    )]
    PostgresOptions {
        uri: String,
        #[source]
        source: sqlx::Error,
    },

    #[error(
        "unknown openid connect provider state storage '{uri}'; expected 'memory' or a 'postgres://' url"
    )]
    UnknownStorage { uri: String },
}
