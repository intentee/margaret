use async_trait::async_trait;
use url::Url;

use margaret::framework::jwks_endpoint::endpoint_error::EndpointError;
use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_jwks_endpoint;
use margaret::framework::macros::singleton;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret::framework::token_trust::token_trust::TokenTrust;

use crate::auth::issuer_identifier::ISSUER_IDENTIFIER;
use crate::auth::token_audience::TOKEN_AUDIENCE;

#[singleton]
#[provides_jwks_endpoint(auth)]
pub struct JwksEndpoint {
    token_trust: TokenTrust,
}

impl JwksEndpoint {
    /// # Errors
    ///
    /// Returns an error when the trusted issuer identifier or audience is malformed.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            token_trust: TokenTrust {
                audience: TOKEN_AUDIENCE.parse()?,
                issuer: ISSUER_IDENTIFIER.parse()?,
            },
        })
    }
}

#[async_trait]
impl ProvidesEndpoint for JwksEndpoint {
    async fn provide(&self) -> anyhow::Result<Url> {
        Url::parse("https://issuer.internal/.well-known/jwks.json").map_err(|source| {
            EndpointError::Resolution {
                source: Box::new(source),
            }
            .into()
        })
    }
}

impl DeclaresTokenTrust for JwksEndpoint {
    fn token_trust(&self) -> &TokenTrust {
        &self.token_trust
    }
}
