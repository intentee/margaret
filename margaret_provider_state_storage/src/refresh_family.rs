use std::collections::BTreeSet;

use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret_oauth_vocabulary::scope::Scope;

use crate::authorization_grant::AuthorizationGrant;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefreshFamily {
    pub auth_time: DateTime<Utc>,
    pub client_id: String,
    pub scopes: BTreeSet<Scope>,
    pub subject: Uuid,
}

impl RefreshFamily {
    #[must_use]
    pub fn opened_by(grant: &AuthorizationGrant) -> Self {
        Self {
            auth_time: grant.auth_time,
            client_id: grant.client_id.clone(),
            scopes: grant.scopes.clone(),
            subject: grant.subject,
        }
    }
}
