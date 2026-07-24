use chrono::DateTime;
use chrono::Utc;

use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct SystemClock;

impl SystemClock {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    #[must_use]
    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
