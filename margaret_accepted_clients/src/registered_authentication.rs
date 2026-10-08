use std::sync::Arc;

use crate::client_key_set::ClientKeySet;
use crate::confidential_privileges::ConfidentialPrivileges;
use crate::remembers_client_assertions::RemembersClientAssertions;

pub enum RegisteredAuthentication {
    None,
    PrivateKeyJwt {
        assertions: Arc<dyn RemembersClientAssertions>,
        keys: Arc<ClientKeySet>,
        privileges: ConfidentialPrivileges,
    },
}
