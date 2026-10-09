use std::sync::Arc;

use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;

use crate::code_grant_policy::CodeGrantPolicy;

pub enum RegisteredCodeGrant {
    Granted {
        grants: Arc<dyn StoresAuthorizationGrants>,
        policy: CodeGrantPolicy,
        redirect_uris: Vec<String>,
    },
    Withheld,
}
