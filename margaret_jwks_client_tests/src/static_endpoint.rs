use async_trait::async_trait;
use url::Url;

use margaret_endpoint::endpoint::Endpoint;
use margaret_endpoint::endpoint_error::EndpointError;
use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

pub struct StaticEndpoint {
    origin: Url,
}

impl StaticEndpoint {
    #[must_use]
    pub fn new(origin: Url) -> Self {
        Self { origin }
    }
}

#[async_trait]
impl ProvidesEndpoint for StaticEndpoint {
    async fn resolve(&self) -> Result<Endpoint, EndpointError> {
        Ok(Endpoint {
            url: self.origin.clone(),
        })
    }
}
