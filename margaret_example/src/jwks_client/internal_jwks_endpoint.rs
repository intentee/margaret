use async_trait::async_trait;
use url::Url;

use margaret_endpoint::endpoint::Endpoint;
use margaret_endpoint::endpoint_error::EndpointError;
use margaret_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_macros::provides_endpoint;
use margaret_macros::singleton;

#[singleton]
#[provides_endpoint(jwks)]
pub struct InternalJwksEndpoint;

#[async_trait]
impl ProvidesEndpoint for InternalJwksEndpoint {
    async fn resolve(&self) -> Result<Endpoint, EndpointError> {
        let url = Url::parse("https://127.0.0.1:9051").map_err(EndpointError::unresolved)?;

        Ok(Endpoint { url })
    }
}
