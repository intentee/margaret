use chrono::DateTime;
use chrono::Utc;

use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::clock::Clock;

#[singleton(provides = Clock)]
pub struct SystemClock;

impl SystemClock {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
    }
}

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
