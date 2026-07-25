use async_trait::async_trait;
use url::Url;

use margaret::framework::endpoint::endpoint_error::EndpointError;
use margaret::framework::endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::macros::provides_jwks_endpoint;
use margaret::framework::macros::singleton;

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
