use margaret_registered_claims::numeric_date::NumericDate;

use crate::contract_moment::contract_moment;

#[must_use]
pub fn contract_instant() -> NumericDate {
    NumericDate::from(contract_moment())
}
