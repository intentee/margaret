use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct MicroservicesMap;

impl MicroservicesMap {
    #[constructor]
    #[must_use]
    pub fn create(#[console_argument(from = "microservices-map")] mapper: Option<String>) -> Self {
        let _ = mapper;

        Self
    }
}

#[singleton]
pub struct MicroservicesMapProvider;

impl MicroservicesMapProvider {
    #[constructor]
    #[must_use]
    pub fn create(map: std::sync::Arc<MicroservicesMap>) -> Self {
        let _ = map;

        Self
    }
}

#[singleton]
pub struct ServiceIdentitySupervisor;

impl ServiceIdentitySupervisor {
    #[constructor]
    #[must_use]
    pub fn create(map: std::sync::Arc<MicroservicesMap>) -> Self {
        let _ = map;

        Self
    }
}

#[singleton]
pub struct SessionValidator;

impl SessionValidator {
    #[constructor]
    #[must_use]
    pub fn create(
        provider: std::sync::Arc<MicroservicesMapProvider>,
        supervisor: std::sync::Arc<ServiceIdentitySupervisor>,
    ) -> Self {
        let _ = (provider, supervisor);

        Self
    }
}

#[singleton]
pub struct RetryChild;

impl RetryChild {
    #[constructor]
    #[must_use]
    pub fn create(#[console_argument(from = "retries")] retries: u16) -> Self {
        let _ = retries;

        Self
    }
}

#[singleton]
pub struct RetrySupervisor;

impl RetrySupervisor {
    #[constructor]
    #[must_use]
    pub fn create(
        #[console_argument(from = "retries")] retries: u16,
        child: std::sync::Arc<RetryChild>,
    ) -> Self {
        let _ = (retries, child);

        Self
    }
}
