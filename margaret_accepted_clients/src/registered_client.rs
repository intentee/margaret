use std::sync::Arc;

use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::accepted_client::AcceptedClient;
use crate::client_key_set::ClientKeySet;
use crate::code_grant_policy::CodeGrantPolicy;
use crate::confidential_privileges::ConfidentialPrivileges;
use crate::registered_authentication::RegisteredAuthentication;
use crate::registered_code_grant::RegisteredCodeGrant;
use crate::remembers_client_assertions::RemembersClientAssertions;

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
        assertions: Arc<dyn RemembersClientAssertions>,
    ) -> Self {
        Self {
            authentication: RegisteredAuthentication::PrivateKeyJwt {
                assertions,
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
        assertions: Arc<dyn RemembersClientAssertions>,
        policy: CodeGrantPolicy,
        redirect_uris: Vec<String>,
        grants: Arc<dyn StoresAuthorizationGrants>,
    ) -> Self {
        Self {
            authentication: RegisteredAuthentication::PrivateKeyJwt {
                assertions,
                keys,
                privileges,
            },
            client,
            code_grant: RegisteredCodeGrant::Granted {
                grants,
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
        policy: CodeGrantPolicy,
        redirect_uris: Vec<String>,
        grants: Arc<dyn StoresAuthorizationGrants>,
    ) -> Self {
        Self {
            authentication: RegisteredAuthentication::None,
            client,
            code_grant: RegisteredCodeGrant::Granted {
                grants,
                policy,
                redirect_uris,
            },
        }
    }
}
