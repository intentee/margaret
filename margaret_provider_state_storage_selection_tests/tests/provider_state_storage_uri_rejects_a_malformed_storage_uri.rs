use std::str::FromStr;

use margaret_provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri;
use margaret_provider_state_storage_selection::provider_state_storage_uri_error::ProviderStateStorageUriError;
use margaret_storage_uri::storage_uri_error::StorageUriError;

#[test]
fn provider_state_storage_uri_rejects_a_malformed_storage_uri() {
    assert!(matches!(
        ProviderStateStorageUri::from_str("memory"),
        Err(ProviderStateStorageUriError::StorageUri(
            StorageUriError::Malformed { .. }
        ))
    ));
}
