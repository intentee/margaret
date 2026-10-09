use std::sync::Arc;

use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::fixture_clients::FixtureClients;
use crate::provider_fixture::ProviderFixture;
use crate::service_client::SERVICE_CLIENT;
use crate::service_privileges::SERVICE_PRIVILEGES;
use crate::started_with_provider_state::started_with_provider_state;

pub async fn serving_unreachable_assertions() -> ProviderFixture {
    let storage = started_with_provider_state().await;

    storage
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "client_assertions",
        )
        .await;

    let clients = FixtureClients {
        asserting: vec![AssertingClient::holding_its_keys(
            SERVICE_CLIENT,
            SERVICE_PRIVILEGES,
            Arc::clone(&storage.database),
        )],
        public: Vec::new(),
    };

    ProviderFixture::serving(storage, clients, Vec::new()).await
}
