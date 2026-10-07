use std::time::Duration;

use margaret_registered_claims::numeric_date::NumericDate;

#[derive(Clone)]
pub(crate) struct SpentAssertion {
    pub(crate) expires_at: NumericDate,
    pub(crate) retained_for: Duration,
}
