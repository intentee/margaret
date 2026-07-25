use async_trait::async_trait;
use url::Url;

use crate::endpoint_error::EndpointError;

#[async_trait]
pub trait ProvidesEndpoint: Send + Sync {
    async fn provide(&self) -> Result<Url, EndpointError>;
}
