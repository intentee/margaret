use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct SessionValidator {
    _provider: Arc<crate::diamond::microservices_map_provider::MicroservicesMapProvider>,
    _supervisor: Arc<crate::diamond::service_identity_supervisor::ServiceIdentitySupervisor>,
}

impl SessionValidator {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        provider: Arc<crate::diamond::microservices_map_provider::MicroservicesMapProvider>,
        supervisor: Arc<crate::diamond::service_identity_supervisor::ServiceIdentitySupervisor>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _provider: provider,
            _supervisor: supervisor,
        })
    }
}
