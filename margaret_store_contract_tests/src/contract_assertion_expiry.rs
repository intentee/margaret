use margaret_accepted_clients::client_assertion_max_lifetime::CLIENT_ASSERTION_MAX_LIFETIME;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::contract_moment::contract_moment;

#[must_use]
pub fn contract_assertion_expiry() -> NumericDate {
    NumericDate::from(contract_moment()).after(CLIENT_ASSERTION_MAX_LIFETIME)
}
