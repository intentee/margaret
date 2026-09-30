use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

use margaret_provider_state_postgres::postgres_provider_state::PostgresProviderState;
use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;

use crate::provider_state_storage_uri::ProviderStateStorageUri;

#[must_use]
pub fn resolve_provider_state_storage(
    uri: ProviderStateStorageUri,
) -> Arc<dyn StoresProviderState> {
    match uri {
        ProviderStateStorageUri::Memory => Arc::new(MemoryProviderState::create()),
        ProviderStateStorageUri::Postgres(options) => Arc::new(PostgresProviderState::create(
            PgPoolOptions::new().connect_lazy_with(*options),
        )),
    }
}
