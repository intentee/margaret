use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::scheduled_with_tick_timer;
use margaret::framework::macros::singleton;

#[singleton]
#[scheduled_with_tick_timer(interval = crate::schedule::PERIOD)]
struct Roller;

impl Roller {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[singleton]
struct Reader;

impl Reader {
    #[constructor]
    fn new(roller: Arc<Roller>) -> anyhow::Result<Self> {}
}
