use std::sync::Arc;

use margaret_database::database::Database;

use crate::client_key_set::ClientKeySet;
use crate::confidential_privileges::ConfidentialPrivileges;

pub enum RegisteredAuthentication {
    None,
    PrivateKeyJwt {
        database: Arc<Database>,
        keys: Arc<ClientKeySet>,
        privileges: ConfidentialPrivileges,
    },
}
