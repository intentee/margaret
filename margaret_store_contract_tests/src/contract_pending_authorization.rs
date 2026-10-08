use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::pending_authorization_lifetime::PENDING_AUTHORIZATION_LIFETIME;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::contract_grant::contract_grant;
use crate::contract_moment::contract_moment;

#[must_use]
pub fn contract_pending_authorization() -> PendingAuthorization {
    PendingAuthorization {
        expires_at: NumericDate::from(contract_moment()).after(PENDING_AUTHORIZATION_LIFETIME),
        grant: contract_grant(),
        state: Some("contract-state".to_string()),
    }
}
