use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct MicroservicesMap;

impl MicroservicesMap {
    #[constructor]
    pub fn create(
        #[console_argument(from = "microservices-map")] mapper: Option<String>,
    ) -> anyhow::Result<Self> {
        Ok({
            let _ = mapper;

            Self
        })
    }
}

#[singleton]
pub struct MicroservicesMapProvider;

impl MicroservicesMapProvider {
    #[constructor]
    pub fn create(map: std::sync::Arc<MicroservicesMap>) -> anyhow::Result<Self> {
        Ok({
            let _ = map;

            Self
        })
    }
}

#[singleton]
pub struct ServiceIdentitySupervisor;

impl ServiceIdentitySupervisor {
    #[constructor]
    pub fn create(map: std::sync::Arc<MicroservicesMap>) -> anyhow::Result<Self> {
        Ok({
            let _ = map;

            Self
        })
    }
}

#[singleton]
pub struct SessionValidator;

impl SessionValidator {
    #[constructor]
    pub fn create(
        provider: std::sync::Arc<MicroservicesMapProvider>,
        supervisor: std::sync::Arc<ServiceIdentitySupervisor>,
    ) -> anyhow::Result<Self> {
        Ok({
            let _ = (provider, supervisor);

            Self
        })
    }
}

#[singleton]
pub struct RetryChild;

impl RetryChild {
    #[constructor]
    pub fn create(#[console_argument(from = "retries")] retries: u16) -> anyhow::Result<Self> {
        Ok({
            let _ = retries;

            Self
        })
    }
}

#[singleton]
pub struct RetrySupervisor;

impl RetrySupervisor {
    #[constructor]
    pub fn create(
        #[console_argument(from = "retries")] retries: u16,
        child: std::sync::Arc<RetryChild>,
    ) -> anyhow::Result<Self> {
        Ok({
            let _ = (retries, child);

            Self
        })
    }
}
