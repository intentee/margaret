use std::collections::BTreeSet;

use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret_oauth_vocabulary::scope::Scope;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grant::AuthorizationGrant;
use crate::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefreshFamily {
    pub auth_time: DateTime<Utc>,
    pub client_id: String,
    pub expires_at: NumericDate,
    pub scopes: BTreeSet<Scope>,
    pub subject: Uuid,
}

impl RefreshFamily {
    #[must_use]
    pub fn opened_by(
        AuthorizationGrant {
            auth_time,
            client_id,
            scopes,
            subject,
            ..
        }: &AuthorizationGrant,
        now: NumericDate,
    ) -> Self {
        Self {
            auth_time: *auth_time,
            client_id: client_id.clone(),
            expires_at: now.after(REFRESH_FAMILY_LIFETIME),
            scopes: scopes.clone(),
            subject: *subject,
        }
    }
}
