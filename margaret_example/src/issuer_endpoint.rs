use async_trait::async_trait;
use url::Url;

use margaret_endpoint::endpoint_error::EndpointError;
use margaret_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_macros::provides_endpoint;
use margaret_macros::singleton;

#[singleton]
#[provides_endpoint(issuer)]
pub struct IssuerEndpoint;

#[async_trait]
impl ProvidesEndpoint for IssuerEndpoint {
    async fn provide(&self) -> Result<Url, EndpointError> {
        Url::parse("https://issuer.internal").map_err(|source| EndpointError::Resolution {
            source: Box::new(source),
        })
    }
}
