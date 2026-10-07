use std::sync::Arc;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;

use crate::accepted_client::AcceptedClient;
use crate::confidential_privileges::ConfidentialPrivileges;

pub enum RegisteredClient {
    Confidential {
        client: AcceptedClient,
        key_set: Arc<IssuerKeySet>,
        privileges: ConfidentialPrivileges,
    },
    Public(AcceptedClient),
}

impl RegisteredClient {
    #[must_use]
    pub fn private_key_jwt(
        client: AcceptedClient,
        privileges: ConfidentialPrivileges,
        key_set: Arc<IssuerKeySet>,
    ) -> Self {
        Self::Confidential {
            client,
            key_set,
            privileges,
        }
    }

    #[must_use]
    pub fn public(client: AcceptedClient) -> Self {
        Self::Public(client)
    }

    #[must_use]
    pub fn client(&self) -> &AcceptedClient {
        match self {
            Self::Confidential { client, .. } | Self::Public(client) => client,
        }
    }
}
