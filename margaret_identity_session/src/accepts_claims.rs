use chrono::DateTime;
use chrono::Utc;

use crate::claims_acceptance::ClaimsAcceptance;

pub trait AcceptsClaims {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    fn accepts(&self, now: DateTime<Utc>) -> anyhow::Result<ClaimsAcceptance>;
}
