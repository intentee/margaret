use async_trait::async_trait;
use url::Url;

use margaret_jwks_endpoint::endpoint_error::EndpointError;
use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_macros::provides_jwks_endpoint;
use margaret_macros::singleton;

#[singleton]
#[provides_jwks_endpoint(auth)]
pub struct JwksEndpoint;

#[async_trait]
impl ProvidesEndpoint for JwksEndpoint {
    async fn provide(&self) -> Result<Url, EndpointError> {
        Url::parse("https://issuer.internal/.well-known/jwks.json").map_err(|source| {
            EndpointError::Resolution {
                source: Box::new(source),
            }
        })
    }
}
