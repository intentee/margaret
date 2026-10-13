use std::sync::Arc;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_jwks_secret_store_tests::fixture_issuance::fixture_issuance;

#[must_use]
pub fn accepted_clients_of(clients: Vec<Arc<RegisteredClient>>) -> AcceptedClients {
    AcceptedClients::create(clients, fixture_issuance())
}
