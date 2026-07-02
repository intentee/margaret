use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::clock::Clock;

#[singleton(provides = Clock)]
pub struct SystemClock;

impl SystemClock {
    #[constructor]
    pub fn create() -> Self {
        Self
    }
}

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("the system clock is set after the unix epoch")
            .as_secs()
    }
}
