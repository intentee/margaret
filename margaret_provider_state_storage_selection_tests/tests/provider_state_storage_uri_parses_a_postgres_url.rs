use std::str::FromStr;

use margaret_provider_state_storage_selection::provider_state_storage_uri::ProviderStateStorageUri;

#[test]
fn provider_state_storage_uri_parses_a_postgres_url() {
    let Ok(ProviderStateStorageUri::Postgres(options)) =
        ProviderStateStorageUri::from_str("postgresql://oidc@database.localhost:5433/provider")
    else {
        panic!("a postgres url selects the postgres storage");
    };

    assert_eq!(options.get_host(), "database.localhost");
    assert_eq!(options.get_port(), 5433);
    assert_eq!(options.get_database(), Some("provider"));
}
