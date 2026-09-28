use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::diamond::microservices_map_provider::MicroservicesMapProvider;
use crate::diamond::service_identity_supervisor::ServiceIdentitySupervisor;

#[singleton]
pub struct SessionValidator {
    _provider: Arc<MicroservicesMapProvider>,
    _supervisor: Arc<ServiceIdentitySupervisor>,
}

impl SessionValidator {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        provider: Arc<MicroservicesMapProvider>,
        supervisor: Arc<ServiceIdentitySupervisor>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _provider: provider,
            _supervisor: supervisor,
        })
    }
}
