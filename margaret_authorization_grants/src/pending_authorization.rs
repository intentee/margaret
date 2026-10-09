use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grant::AuthorizationGrant;
use crate::pending_authorization_lifetime::PENDING_AUTHORIZATION_LIFETIME;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingAuthorization {
    pub expires_at: NumericDate,
    pub grant: AuthorizationGrant,
    pub state: Option<String>,
}

impl PendingAuthorization {
    #[must_use]
    pub fn held_at(now: NumericDate, grant: AuthorizationGrant, state: Option<String>) -> Self {
        Self {
            expires_at: now.after(PENDING_AUTHORIZATION_LIFETIME),
            grant,
            state,
        }
    }
}
