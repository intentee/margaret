use std::sync::Arc;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::declares_accepted_client::DeclaresAcceptedClient;
use margaret_accepted_clients_tests::accepted_client_declaration::AcceptedClientDeclaration;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;

use crate::portal_client::portal_client;
use crate::provider_issuance::provider_issuance;

pub struct UnrolledProvider {
    pub clients: Arc<AcceptedClients>,
    pub issuance: Arc<dyn DeclaresTokenIssuance>,
    pub secret_store: Arc<JwksSecretStore>,
}

impl UnrolledProvider {
    pub fn create() -> Self {
        let issuance: Arc<dyn DeclaresTokenIssuance> = Arc::new(provider_issuance());

        Self {
            clients: Arc::new(
                AcceptedClients::create(
                    vec![Arc::new(AcceptedClientDeclaration {
                        client: portal_client(),
                    }) as Arc<dyn DeclaresAcceptedClient>],
                    issuance.as_ref(),
                )
                .expect("the portal client is accepted"),
            ),
            secret_store: Arc::new(JwksSecretStore::create(
                fixture_roller(),
                Arc::clone(&issuance),
            )),
            issuance,
        }
    }
}
