use std::sync::Arc;

use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_accepted_clients::registered_code_grant::RegisteredCodeGrant;
use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::fixture_client_assertions::FixtureClientAssertions;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::fixture_authorization_grants::FixtureAuthorizationGrants;
use crate::portal_callback::PORTAL_CALLBACK;
use crate::portal_client::PORTAL_CLIENT;
use crate::portal_code_grant::PORTAL_CODE_GRANT;
use crate::portal_privileges::PORTAL_PRIVILEGES;
use crate::service_client::SERVICE_CLIENT;
use crate::service_privileges::SERVICE_PRIVILEGES;
use crate::spa_callback::SPA_CALLBACK;
use crate::spa_client::SPA_CLIENT;
use crate::spa_code_grant::SPA_CODE_GRANT;

pub struct FixtureClients {
    pub asserting: Vec<AssertingClient>,
    pub grants: Arc<dyn StoresAuthorizationGrants>,
    pub public: Vec<Arc<RegisteredClient>>,
}

impl FixtureClients {
    pub async fn over_grants(grants: Arc<dyn StoresAuthorizationGrants>) -> Self {
        let assertions: Arc<dyn RemembersClientAssertions> =
            Arc::new(FixtureClientAssertions::default());

        Self {
            asserting: vec![
                AssertingClient::holding_its_keys(
                    PORTAL_CLIENT,
                    PORTAL_PRIVILEGES,
                    Arc::clone(&assertions),
                    RegisteredCodeGrant::Granted {
                        grants: Arc::clone(&grants),
                        policy: PORTAL_CODE_GRANT,
                        redirect_uris: vec![PORTAL_CALLBACK.to_string()],
                    },
                )
                .await,
                AssertingClient::holding_its_keys(
                    SERVICE_CLIENT,
                    SERVICE_PRIVILEGES,
                    assertions,
                    RegisteredCodeGrant::Withheld,
                )
                .await,
            ],
            public: vec![Arc::new(RegisteredClient::public_with_code_grant(
                SPA_CLIENT,
                SPA_CODE_GRANT,
                vec![SPA_CALLBACK.to_string()],
                Arc::clone(&grants),
            ))],
            grants,
        }
    }

    pub async fn standard() -> Self {
        Self::over_grants(Arc::new(FixtureAuthorizationGrants::undisturbed())).await
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
