use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::diamond::retry_child::RetryChild;

#[singleton]
pub struct RetrySupervisor {
    _child: Arc<RetryChild>,
    _retries: u16,
}

impl RetrySupervisor {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[console_argument(from = "retries")] retries: u16,
        child: Arc<RetryChild>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _child: child,
            _retries: retries,
        })
    }
}
