use margaret::framework::macros::process;
use margaret::framework::macros::scheduled_with_tick_timer;

#[scheduled_with_tick_timer(interval = crate::interval::INTERVAL)]
pub struct Pulse;

impl Pulse {
    #[process]
    /// # Errors
    ///
    /// Returns `Error` propagated from the work it performs.
    pub fn run(&self) -> core::result::Result<(), failures::Error> {
        Ok(())
    }
}
