use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::diamond::microservices_map::MicroservicesMap;

#[singleton]
pub struct ServiceIdentitySupervisor {
    _map: Arc<MicroservicesMap>,
}

impl ServiceIdentitySupervisor {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(map: Arc<MicroservicesMap>) -> anyhow::Result<Self> {
        Ok(Self { _map: map })
    }
}
