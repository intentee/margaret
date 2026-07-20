use async_trait::async_trait;

use crate::endpoint::Endpoint;
use crate::endpoint_error::EndpointError;

#[async_trait]
pub trait ProvidesEndpoint: Send + Sync {
    async fn resolve(&self) -> Result<Endpoint, EndpointError>;
}
