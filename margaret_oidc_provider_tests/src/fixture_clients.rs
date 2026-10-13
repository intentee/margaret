use std::sync::Arc;

use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_database::database::Database;

use crate::ledger_client::LEDGER_CLIENT;
use crate::ledger_privileges::LEDGER_PRIVILEGES;
use crate::portal_callback::PORTAL_CALLBACK;
use crate::portal_client::PORTAL_CLIENT;
use crate::portal_code_grant::PORTAL_CODE_GRANT;
use crate::portal_privileges::PORTAL_PRIVILEGES;
use crate::service_client::SERVICE_CLIENT;
use crate::service_privileges::SERVICE_PRIVILEGES;
use crate::spa_callback::SPA_CALLBACK;
use crate::spa_client::SPA_CLIENT;
use crate::spa_code_grant::SPA_CODE_GRANT;
use crate::staff_callback::STAFF_CALLBACK;
use crate::staff_client::STAFF_CLIENT;
use crate::staff_code_grant::STAFF_CODE_GRANT;
use crate::staff_privileges::STAFF_PRIVILEGES;

pub struct FixtureClients {
    pub asserting: Vec<AssertingClient>,
    pub public: Vec<Arc<RegisteredClient>>,
}

impl FixtureClients {
    #[must_use]
    pub fn standard(database: &Arc<Database>) -> Self {
        Self {
            asserting: vec![
                AssertingClient::holding_its_keys_with_code_grant(
                    PORTAL_CLIENT,
                    PORTAL_PRIVILEGES,
                    Arc::clone(database),
                    PORTAL_CODE_GRANT,
                    vec![PORTAL_CALLBACK.to_string()],
                ),
                AssertingClient::holding_its_keys(
                    SERVICE_CLIENT,
                    SERVICE_PRIVILEGES,
                    Arc::clone(database),
                ),
                AssertingClient::holding_its_keys(
                    LEDGER_CLIENT,
                    LEDGER_PRIVILEGES,
                    Arc::clone(database),
                ),
                AssertingClient::holding_its_keys_with_code_grant(
                    STAFF_CLIENT,
                    STAFF_PRIVILEGES,
                    Arc::clone(database),
                    STAFF_CODE_GRANT,
                    vec![STAFF_CALLBACK.to_string()],
                ),
            ],
            public: vec![Arc::new(RegisteredClient::public_with_code_grant(
                SPA_CLIENT,
                Arc::clone(database),
                SPA_CODE_GRANT,
                vec![SPA_CALLBACK.to_string()],
            ))],
        }
    }

    /// # Panics
    ///
    /// Panics when no asserting client carries the client identifier.
    #[must_use]
    pub fn asserting(&self, client_id: &str) -> &AssertingClient {
        self.asserting
            .iter()
            .find(|asserting| asserting.registered.client.client_id == client_id)
            .expect("an asserting client carries the client identifier")
    }

    #[must_use]
    pub fn registered(&self) -> Vec<Arc<RegisteredClient>> {
        self.asserting
            .iter()
            .map(|asserting| Arc::clone(&asserting.registered))
            .chain(self.public.iter().cloned())
            .collect()
    }
}
