use std::str::FromStr;

use margaret_provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri;
use margaret_provider_state_storage_selection::provider_state_storage_uri_error::ProviderStateStorageUriError;

#[test]
fn provider_state_storage_uri_rejects_a_file_storage() {
    assert!(matches!(
        ProviderStateStorageUri::from_str("file:///var/lib/app/provider"),
        Err(ProviderStateStorageUriError::FileUnsupported)
    ));
}
