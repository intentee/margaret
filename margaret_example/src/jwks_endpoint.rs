use async_trait::async_trait;
use url::Url;

use margaret::framework::jwks_endpoint::endpoint_error::EndpointError;
use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;
use margaret::framework::jwt_claims::expected_claims::ExpectedClaims;
use margaret::framework::jwt_claims::provides_expected_claims::ProvidesExpectedClaims;
use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_jwks_endpoint;
use margaret::framework::macros::singleton;

use crate::access_token_audience::ACCESS_TOKEN_AUDIENCE;
use crate::access_token_issuer::ACCESS_TOKEN_ISSUER;

#[singleton]
#[provides_jwks_endpoint(auth)]
pub struct JwksEndpoint {
    endpoint: Url,
}

impl JwksEndpoint {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            endpoint: Url::parse(&format!("{ACCESS_TOKEN_ISSUER}{WELL_KNOWN_JWKS_PATH}")).map_err(
                |source| EndpointError::Resolution {
                    source: Box::new(source),
                },
            )?,
        })
    }
}

#[async_trait]
impl ProvidesEndpoint for JwksEndpoint {
    async fn provide(&self) -> anyhow::Result<Url> {
        Ok(self.endpoint.clone())
    }
}

impl ProvidesExpectedClaims for JwksEndpoint {
    fn expected_claims(&self) -> ExpectedClaims {
        ExpectedClaims {
            audience: ACCESS_TOKEN_AUDIENCE.to_string(),
            issuer: ACCESS_TOKEN_ISSUER.to_string(),
        }
    }
}
