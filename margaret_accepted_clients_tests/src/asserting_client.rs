use std::sync::Arc;

use serde_json::Value;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;

pub struct AssertingClient {
    pub registered: Arc<RegisteredClient>,
    pub roller: Arc<JwksRoller>,
}

impl AssertingClient {
    #[must_use]
    pub fn awaiting_its_keys(client: AcceptedClient, privileges: ConfidentialPrivileges) -> Self {
        Self::with_key_set(
            client,
            privileges,
            fixture_roller(),
            IssuerKeySet::awaiting(),
        )
    }

    #[must_use]
    pub fn holding_its_keys(client: AcceptedClient, privileges: ConfidentialPrivileges) -> Self {
        let roller = fixture_roller();
        let key_set = IssuerKeySet::awaiting();

        key_set.hold(Arc::new(
            roller.jwks_secret_holder().get().key_set().clone(),
        ));

        Self::with_key_set(client, privileges, roller, key_set)
    }

    #[must_use]
    pub fn assertion(&self, claims: &Value) -> String {
        self.roller
            .jwks_secret_holder()
            .get()
            .current()
            .sign_json(claims, JwtType::ClientAuthentication)
    }

    fn with_key_set(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        roller: Arc<JwksRoller>,
        key_set: IssuerKeySet,
    ) -> Self {
        Self {
            registered: Arc::new(RegisteredClient::private_key_jwt(
                client,
                privileges,
                Arc::new(key_set),
            )),
            roller,
        }
    }
}
