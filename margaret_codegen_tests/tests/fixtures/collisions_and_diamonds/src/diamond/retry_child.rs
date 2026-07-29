use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct RetryChild {
    _retries: u16,
}

impl RetryChild {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create(#[console_argument(from = "retries")] retries: u16) -> anyhow::Result<Self> {
        Ok(Self { _retries: retries })
    }
}
