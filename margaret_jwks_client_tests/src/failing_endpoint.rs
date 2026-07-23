use async_trait::async_trait;
use url::Url;

use margaret_endpoint::endpoint_error::EndpointError;
use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

pub struct FailingEndpoint;

#[async_trait]
impl ProvidesEndpoint for FailingEndpoint {
    async fn provide(&self) -> Result<Url, EndpointError> {
        Err(EndpointError::Resolution {
            source: "the issuer endpoint could not be discovered".into(),
        })
    }
}
