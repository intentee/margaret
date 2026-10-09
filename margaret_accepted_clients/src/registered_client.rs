use std::sync::Arc;

use margaret_database::database::Database;

use crate::accepted_client::AcceptedClient;
use crate::client_key_set::ClientKeySet;
use crate::code_grant_policy::CodeGrantPolicy;
use crate::confidential_privileges::ConfidentialPrivileges;
use crate::registered_authentication::RegisteredAuthentication;
use crate::registered_code_grant::RegisteredCodeGrant;

pub struct RegisteredClient {
    pub authentication: RegisteredAuthentication,
    pub client: AcceptedClient,
    pub code_grant: RegisteredCodeGrant,
}

impl RegisteredClient {
    #[must_use]
    pub fn private_key_jwt(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        keys: Arc<ClientKeySet>,
        database: Arc<Database>,
    ) -> Self {
        Self {
            authentication: RegisteredAuthentication::PrivateKeyJwt {
                database,
                keys,
                privileges,
            },
            client,
            code_grant: RegisteredCodeGrant::Withheld,
        }
    }

    #[must_use]
    pub fn private_key_jwt_with_code_grant(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        keys: Arc<ClientKeySet>,
        database: Arc<Database>,
        policy: CodeGrantPolicy,
        redirect_uris: Vec<String>,
    ) -> Self {
        Self {
            authentication: RegisteredAuthentication::PrivateKeyJwt {
                database: Arc::clone(&database),
                keys,
                privileges,
            },
            client,
            code_grant: RegisteredCodeGrant::Granted {
                database,
                policy,
                redirect_uris,
            },
        }
    }

    #[must_use]
    pub fn public(client: AcceptedClient) -> Self {
        Self {
            authentication: RegisteredAuthentication::None,
            client,
            code_grant: RegisteredCodeGrant::Withheld,
        }
    }

    #[must_use]
    pub fn public_with_code_grant(
        client: AcceptedClient,
        database: Arc<Database>,
        policy: CodeGrantPolicy,
        redirect_uris: Vec<String>,
    ) -> Self {
        Self {
            authentication: RegisteredAuthentication::None,
            client,
            code_grant: RegisteredCodeGrant::Granted {
                database,
                policy,
                redirect_uris,
            },
        }
    }
}
