use std::collections::BTreeSet;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::scope::Scope;

pub struct IntrospectedToken<TClaims> {
    pub claims: TClaims,
    pub client_id: Option<ClientId>,
    pub scopes: Option<BTreeSet<Scope>>,
    pub subject: Option<String>,
    pub username: Option<String>,
}
