use std::str::FromStr;

use margaret_provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri;
use margaret_provider_state_storage_selection::provider_state_storage_uri_error::ProviderStateStorageUriError;

#[test]
fn provider_state_storage_uri_rejects_an_unknown_storage() {
    for uri in ["vault", "vault://secrets.localhost/provider"] {
        let rejection =
            ProviderStateStorageUri::from_str(uri).expect_err("an unknown storage is rejected");

        assert!(
            matches!(&rejection, ProviderStateStorageUriError::UnknownStorage { uri: rejected } if rejected == uri)
        );
        assert!(rejection.to_string().contains(&format!("'{uri}'")));
    }
}
