use chrono::DateTime;
use chrono::Utc;

use margaret_identity::clock::Clock;

pub struct FixedClock {
    now: DateTime<Utc>,
}

impl FixedClock {
    #[must_use]
    pub fn new(now: DateTime<Utc>) -> Self {
        Self { now }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.now
    }
}
