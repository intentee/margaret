use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct ServiceIdentitySupervisor {
    _map: Arc<crate::diamond::microservices_map::MicroservicesMap>,
}

impl ServiceIdentitySupervisor {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create(
        map: Arc<crate::diamond::microservices_map::MicroservicesMap>,
    ) -> anyhow::Result<Self> {
        Ok(Self { _map: map })
    }
}
