use std::sync::Arc;

use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_database::database::Database;

use crate::constants::LEDGER_CLIENT;
use crate::constants::LEDGER_PRIVILEGES;
use crate::constants::PORTAL_CALLBACK;
use crate::constants::PORTAL_CLIENT;
use crate::constants::PORTAL_CODE_GRANT;
use crate::constants::PORTAL_PRIVILEGES;
use crate::constants::SERVICE_CLIENT;
use crate::constants::SERVICE_PRIVILEGES;
use crate::constants::SPA_CALLBACK;
use crate::constants::SPA_CLIENT;
use crate::constants::SPA_CODE_GRANT;
use crate::constants::STAFF_CALLBACK;
use crate::constants::STAFF_CLIENT;
use crate::constants::STAFF_CODE_GRANT;
use crate::constants::STAFF_PRIVILEGES;

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
