use std::time::Duration;

use chrono::Utc;

use margaret_registered_claims::numeric_date::NumericDate;

pub struct AssertionClock {
    pub now: NumericDate,
}

impl AssertionClock {
    #[must_use]
    pub fn start() -> Self {
        Self {
            now: NumericDate::from(Utc::now()),
        }
    }

    #[must_use]
    pub fn in_seconds(&self, seconds: u64) -> NumericDate {
        self.now.after(Duration::from_secs(seconds))
    }
}
