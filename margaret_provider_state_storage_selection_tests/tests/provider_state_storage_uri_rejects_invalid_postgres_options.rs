use std::str::FromStr;

use margaret_provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri;
use margaret_provider_state_storage_selection::provider_state_storage_uri_error::ProviderStateStorageUriError;

#[test]
fn provider_state_storage_uri_rejects_invalid_postgres_options() {
    let rejection = ProviderStateStorageUri::from_str(
        "postgres://oidc:hunter2@database.localhost/provider?sslmode=sometimes",
    )
    .err()
    .expect("an unknown ssl mode is rejected");

    assert!(matches!(
        rejection,
        ProviderStateStorageUriError::PostgresOptions { .. }
    ));
    assert!(!rejection.to_string().contains("hunter2"));
}
