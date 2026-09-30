use std::collections::BTreeSet;

use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::scope::Scope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefreshFamily {
    pub auth_time: DateTime<Utc>,
    pub client_id: ClientId,
    pub id: Uuid,
    pub scopes: BTreeSet<Scope>,
    pub subject: Uuid,
}
