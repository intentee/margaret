use std::collections::BTreeSet;

use uuid::Uuid;

use margaret_oauth_vocabulary::scope::Scope;

#[derive(Debug, Eq, PartialEq)]
pub struct ConsentRequest {
    pub client_id: &'static str,
    pub id: Uuid,
    pub scopes: BTreeSet<Scope>,
}
