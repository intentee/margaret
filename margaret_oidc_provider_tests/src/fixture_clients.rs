use std::sync::Arc;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;

use crate::portal_client::PORTAL_CLIENT;
use crate::portal_privileges::PORTAL_PRIVILEGES;
use crate::service_client::SERVICE_CLIENT;
use crate::service_privileges::SERVICE_PRIVILEGES;
use crate::spa_client::SPA_CLIENT;

pub struct FixtureClients {
    pub asserting: Vec<AssertingClient>,
    pub public: Vec<AcceptedClient>,
}

impl FixtureClients {
    #[must_use]
    pub fn standard() -> Self {
        Self {
            asserting: vec![
                AssertingClient::holding_its_keys(PORTAL_CLIENT, PORTAL_PRIVILEGES),
                AssertingClient::holding_its_keys(SERVICE_CLIENT, SERVICE_PRIVILEGES),
            ],
            public: vec![SPA_CLIENT],
        }
    }

    /// # Panics
    ///
    /// Panics when no asserting client carries the client identifier.
    #[must_use]
    pub fn asserting(&self, client_id: &str) -> &AssertingClient {
        self.asserting
            .iter()
            .find(|asserting| asserting.registered.client().client_id == client_id)
            .expect("an asserting client carries the client identifier")
    }

    #[must_use]
    pub fn registered(&self) -> Vec<Arc<RegisteredClient>> {
        self.asserting
            .iter()
            .map(|asserting| Arc::clone(&asserting.registered))
            .chain(
                self.public
                    .iter()
                    .map(|client| Arc::new(RegisteredClient::public(*client))),
            )
            .collect()
    }
}
