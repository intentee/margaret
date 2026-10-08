use std::sync::Arc;

use serde_json::Value;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::client_key_set::ClientKeySet;
use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_database::database::Database;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_secrets::fixture_secrets;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;

fn held(secrets: &JwksSecretHolder) -> Arc<ClientKeySet> {
    let key_set = IssuerKeySet::awaiting();

    key_set.hold(Arc::new(secrets.get().published_key_set().clone()));

    published(key_set)
}

fn published(key_set: IssuerKeySet) -> Arc<ClientKeySet> {
    Arc::new(ClientKeySet::published(
        Arc::new(key_set),
        JwsAlgorithm::Es256,
    ))
}

pub struct AssertingClient {
    pub registered: Arc<RegisteredClient>,
    pub secrets: Arc<JwksSecretHolder>,
}

impl AssertingClient {
    #[must_use]
    pub fn awaiting_its_keys(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        database: Arc<Database>,
    ) -> Self {
        Self {
            registered: Arc::new(RegisteredClient::private_key_jwt(
                client,
                privileges,
                published(IssuerKeySet::awaiting()),
                database,
            )),
            secrets: fixture_secrets(),
        }
    }

    #[must_use]
    pub fn holding_its_keys(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        database: Arc<Database>,
    ) -> Self {
        let secrets = fixture_secrets();

        Self {
            registered: Arc::new(RegisteredClient::private_key_jwt(
                client,
                privileges,
                held(&secrets),
                database,
            )),
            secrets,
        }
    }

    #[must_use]
    pub fn holding_its_keys_with_code_grant(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        database: Arc<Database>,
        policy: CodeGrantPolicy,
        redirect_uris: Vec<String>,
    ) -> Self {
        let secrets = fixture_secrets();

        Self {
            registered: Arc::new(RegisteredClient::private_key_jwt_with_code_grant(
                client,
                privileges,
                held(&secrets),
                database,
                policy,
                redirect_uris,
            )),
            secrets,
        }
    }

    #[must_use]
    pub fn signing_with_own_keys(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        database: Arc<Database>,
    ) -> Self {
        let secrets = fixture_secrets();
        let keys = Arc::new(ClientKeySet::own(Arc::new(JwksSecretStore::create(
            Arc::clone(&secrets),
            fixture_issuance(),
        ))));

        Self {
            registered: Arc::new(RegisteredClient::private_key_jwt(
                client, privileges, keys, database,
            )),
            secrets,
        }
    }

    #[must_use]
    pub fn assertion(&self, claims: &Value) -> String {
        self.secrets
            .get()
            .current()
            .sign_json(claims, JwtType::ClientAuthentication)
    }
}
