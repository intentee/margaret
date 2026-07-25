use chrono::DateTime;
use chrono::Utc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

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
