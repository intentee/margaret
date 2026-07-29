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
