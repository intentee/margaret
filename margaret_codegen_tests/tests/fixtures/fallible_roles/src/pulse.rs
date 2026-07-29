use margaret::framework::macros::process;
use margaret::framework::macros::scheduled_with_tick_timer;

#[scheduled_with_tick_timer(interval = crate::interval::INTERVAL)]
pub struct Pulse;

impl Pulse {
    /// # Errors
    ///
    /// Returns `Error` propagated from the work it performs.
    #[process]
    pub fn run(&self) -> core::result::Result<(), failures::Error> {
        Ok(())
    }
}
