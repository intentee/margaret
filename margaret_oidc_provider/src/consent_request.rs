use std::collections::BTreeSet;

use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::scope::Scope;

#[derive(Debug, Eq, PartialEq)]
pub struct ConsentRequest {
    pub client_id: ClientId,
    pub id: Uuid,
    pub scopes: BTreeSet<Scope>,
}
