use std::str::FromStr;

use margaret_provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri;

#[test]
fn provider_state_storage_uri_parses_the_memory_storage() {
    assert!(matches!(
        ProviderStateStorageUri::from_str("memory:"),
        Ok(ProviderStateStorageUri::Memory)
    ));
}
