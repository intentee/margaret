use std::sync::Arc;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_accepted_clients::declares_accepted_client::DeclaresAcceptedClient;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;

use crate::accepted_client_declaration::AcceptedClientDeclaration;

/// # Errors
///
/// Returns the `AcceptedClientsError` the registry reports for the clients.
pub fn accepted_clients_of(
    clients: Vec<AcceptedClient>,
) -> Result<AcceptedClients, AcceptedClientsError> {
    AcceptedClients::create(
        clients
            .into_iter()
            .map(|client| {
                Arc::new(AcceptedClientDeclaration { client }) as Arc<dyn DeclaresAcceptedClient>
            })
            .collect(),
        &fixture_issuance(),
    )
}
