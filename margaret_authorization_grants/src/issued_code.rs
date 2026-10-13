use margaret_registered_claims::numeric_date::NumericDate;

use crate::authorization_grant::AuthorizationGrant;
use crate::constants::AUTHORIZATION_CODE_LIFETIME;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuedCode {
    pub expires_at: NumericDate,
    pub grant: AuthorizationGrant,
}

impl IssuedCode {
    #[must_use]
    pub fn issued_at(now: NumericDate, grant: AuthorizationGrant) -> Self {
        Self {
            expires_at: now.after(AUTHORIZATION_CODE_LIFETIME),
            grant,
        }
    }
}
