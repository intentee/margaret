use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct RetrySupervisor {
    _child: Arc<crate::diamond::retry_child::RetryChild>,
    _retries: u16,
}

impl RetrySupervisor {
    #[constructor]
    pub fn create(
        #[console_argument(from = "retries")] retries: u16,
        child: Arc<crate::diamond::retry_child::RetryChild>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _child: child,
            _retries: retries,
        })
    }
}
