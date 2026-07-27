use std::time::Duration;

use margaret::framework::macros::process;
use margaret::framework::macros::scheduled_with_tick_timer;

pub(crate) const INTERVAL: Duration = Duration::from_secs(60);

#[scheduled_with_tick_timer(interval = crate::pulse::INTERVAL)]
pub struct Pulse;

impl Pulse {
    #[process]
    pub async fn run(&self) -> core::result::Result<(), failures::Error> {
        Ok(())
    }
}
