use std::io::Error;
use std::io::ErrorKind;

use async_trait::async_trait;

use margaret_endpoint::endpoint::Endpoint;
use margaret_endpoint::endpoint_error::EndpointError;
use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

pub struct FailingEndpoint;

#[async_trait]
impl ProvidesEndpoint for FailingEndpoint {
    async fn resolve(&self) -> Result<Endpoint, EndpointError> {
        Err(EndpointError::unresolved(Error::from(
            ErrorKind::NotConnected,
        )))
    }
}
