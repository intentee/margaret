use std::sync::Arc;

use margaret_accepted_clients::registered_code_grant::RegisteredCodeGrant;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::unreachable_client_assertions::UnreachableClientAssertions;

use crate::fixture_authorization_grants::FixtureAuthorizationGrants;
use crate::fixture_clients::FixtureClients;
use crate::provider_fixture::ProviderFixture;
use crate::service_client::SERVICE_CLIENT;
use crate::service_privileges::SERVICE_PRIVILEGES;

pub async fn serving_unreachable_assertions() -> ProviderFixture {
    ProviderFixture::serving(
        FixtureClients {
            asserting: vec![
                AssertingClient::holding_its_keys(
                    SERVICE_CLIENT,
                    SERVICE_PRIVILEGES,
                    Arc::new(UnreachableClientAssertions),
                    RegisteredCodeGrant::Withheld,
                )
                .await,
            ],
            grants: Arc::new(FixtureAuthorizationGrants::undisturbed()),
            public: Vec::new(),
        },
        Vec::new(),
    )
    .await
}
