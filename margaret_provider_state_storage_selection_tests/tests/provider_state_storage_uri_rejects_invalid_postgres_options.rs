use std::str::FromStr;

use margaret_provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri;
use margaret_provider_state_storage_selection::provider_state_storage_uri_error::ProviderStateStorageUriError;

#[test]
fn provider_state_storage_uri_rejects_invalid_postgres_options() {
    let rejection = ProviderStateStorageUri::from_str(
        "postgres://database.localhost/provider?sslmode=sometimes",
    )
    .expect_err("an unknown ssl mode is rejected");

    assert!(matches!(
        &rejection,
        ProviderStateStorageUriError::PostgresOptions { uri, .. }
            if uri == "postgres://database.localhost/provider?sslmode=sometimes"
    ));
    assert!(
        rejection
            .to_string()
            .starts_with("the openid connect provider state storage 'postgres://database.localhost/provider?sslmode=sometimes' is not valid postgres options: ")
    );
}
