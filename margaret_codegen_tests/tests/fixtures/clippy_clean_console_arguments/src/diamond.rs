use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct MicroservicesMap {
    _mapper: Option<String>,
}

impl MicroservicesMap {
    #[constructor]
    pub fn create(
        #[console_argument(from = "microservices-map")] mapper: Option<String>,
    ) -> anyhow::Result<Self> {
        Ok(Self { _mapper: mapper })
    }
}

#[singleton]
pub struct MicroservicesMapProvider {
    _map: std::sync::Arc<MicroservicesMap>,
}

impl MicroservicesMapProvider {
    #[constructor]
    pub fn create(map: std::sync::Arc<MicroservicesMap>) -> anyhow::Result<Self> {
        Ok(Self { _map: map })
    }
}

#[singleton]
pub struct ServiceIdentitySupervisor {
    _map: std::sync::Arc<MicroservicesMap>,
}

impl ServiceIdentitySupervisor {
    #[constructor]
    pub fn create(map: std::sync::Arc<MicroservicesMap>) -> anyhow::Result<Self> {
        Ok(Self { _map: map })
    }
}

#[singleton]
pub struct SessionValidator {
    _provider: std::sync::Arc<MicroservicesMapProvider>,
    _supervisor: std::sync::Arc<ServiceIdentitySupervisor>,
}

impl SessionValidator {
    #[constructor]
    pub fn create(
        provider: std::sync::Arc<MicroservicesMapProvider>,
        supervisor: std::sync::Arc<ServiceIdentitySupervisor>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _provider: provider,
            _supervisor: supervisor,
        })
    }
}

#[singleton]
pub struct RetryChild {
    _retries: u16,
}

impl RetryChild {
    #[constructor]
    pub fn create(#[console_argument(from = "retries")] retries: u16) -> anyhow::Result<Self> {
        Ok(Self { _retries: retries })
    }
}

#[singleton]
pub struct RetrySupervisor {
    _child: std::sync::Arc<RetryChild>,
    _retries: u16,
}

impl RetrySupervisor {
    #[constructor]
    pub fn create(
        #[console_argument(from = "retries")] retries: u16,
        child: std::sync::Arc<RetryChild>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _child: child,
            _retries: retries,
        })
    }
}
