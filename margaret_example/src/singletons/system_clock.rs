use chrono::DateTime;
use chrono::Utc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct SystemClock;

impl SystemClock {
    #[must_use]
    #[constructor]
    pub fn create() -> Self {
        Self
    }

    #[must_use]
    pub fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
