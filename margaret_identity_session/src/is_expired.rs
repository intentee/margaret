use chrono::DateTime;
use chrono::Utc;

pub trait IsExpired {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    fn is_expired(&self, now: DateTime<Utc>) -> anyhow::Result<bool>;
}
