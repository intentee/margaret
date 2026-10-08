use std::sync::Arc;

use serde_json::Value;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::client_key_set::ClientKeySet;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_accepted_clients::registered_code_grant::RegisteredCodeGrant;
use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;

fn published(key_set: IssuerKeySet) -> Arc<ClientKeySet> {
    Arc::new(ClientKeySet::published(
        Arc::new(key_set),
        JwsAlgorithm::Es256,
    ))
}

fn registered(
    client: AcceptedClient,
    privileges: ConfidentialPrivileges,
    keys: Arc<ClientKeySet>,
    assertions: Arc<dyn RemembersClientAssertions>,
    code_grant: RegisteredCodeGrant,
) -> Arc<RegisteredClient> {
    Arc::new(match code_grant {
        RegisteredCodeGrant::Granted {
            grants,
            policy,
            redirect_uris,
        } => RegisteredClient::private_key_jwt_with_code_grant(
            client,
            privileges,
            keys,
            assertions,
            policy,
            redirect_uris,
            grants,
        ),
        RegisteredCodeGrant::Withheld => {
            RegisteredClient::private_key_jwt(client, privileges, keys, assertions)
        }
    })
}

pub struct AssertingClient {
    pub registered: Arc<RegisteredClient>,
    pub roller: Arc<JwksRoller>,
}

impl AssertingClient {
    pub async fn awaiting_its_keys(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        assertions: Arc<dyn RemembersClientAssertions>,
        code_grant: RegisteredCodeGrant,
    ) -> Self {
        Self {
            registered: registered(
                client,
                privileges,
                published(IssuerKeySet::awaiting()),
                assertions,
                code_grant,
            ),
            roller: fixture_roller().await,
        }
    }

    pub async fn holding_its_keys(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        assertions: Arc<dyn RemembersClientAssertions>,
        code_grant: RegisteredCodeGrant,
    ) -> Self {
        let roller = fixture_roller().await;
        let key_set = IssuerKeySet::awaiting();

        key_set.hold(Arc::new(
            roller.jwks_secret_holder().get().key_set().clone(),
        ));

        Self {
            registered: registered(
                client,
                privileges,
                published(key_set),
                assertions,
                code_grant,
            ),
            roller,
        }
    }

    pub async fn signing_with_own_keys(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        assertions: Arc<dyn RemembersClientAssertions>,
    ) -> Self {
        let roller = fixture_roller().await;
        let keys = Arc::new(ClientKeySet::own(Arc::new(JwksSecretStore::create(
            Arc::clone(&roller),
            fixture_issuance(),
        ))));

        Self {
            registered: registered(
                client,
                privileges,
                keys,
                assertions,
                RegisteredCodeGrant::Withheld,
            ),
            roller,
        }
    }

    #[must_use]
    pub fn assertion(&self, claims: &Value) -> String {
        self.roller
            .jwks_secret_holder()
            .get()
            .current()
            .sign_json(claims, JwtType::ClientAuthentication)
    }
}
