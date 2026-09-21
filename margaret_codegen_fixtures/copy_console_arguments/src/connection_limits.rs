use std::num::NonZeroU32;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::budget::Budget;

#[singleton]
pub struct ConnectionLimits {
    pub budget: Budget,
    pub max_connections: NonZeroU32,
}

impl ConnectionLimits {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[console_argument(from = "budget")] budget: Budget,
        #[console_argument(from = "max-connections")] max_connections: NonZeroU32,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            budget,
            max_connections,
        })
    }
}
