use std::collections::BTreeSet;

use uuid::Uuid;

use margaret_oauth_vocabulary::scope::Scope;

#[derive(Debug, Eq, PartialEq)]
pub struct UserinfoGrant {
    pub scopes: BTreeSet<Scope>,
    pub subject: Uuid,
}
