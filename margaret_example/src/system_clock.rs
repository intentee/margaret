use chrono::DateTime;
use chrono::Utc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct SystemClock;

impl SystemClock {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
    }

    #[must_use]
    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
