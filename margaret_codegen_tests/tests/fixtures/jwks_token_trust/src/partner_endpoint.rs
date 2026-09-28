use async_trait::async_trait;
use url::Url;

use margaret::framework::jwks_endpoint::endpoint_error::EndpointError;
use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_jwks_endpoint;
use margaret::framework::macros::singleton;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret::framework::token_trust::token_trust::TokenTrust;

#[singleton]
#[provides_jwks_endpoint(partner)]
pub struct PartnerEndpoint {
    token_trust: TokenTrust,
}

impl PartnerEndpoint {
    /// # Errors
    ///
    /// Returns an error when the trusted issuer identifier or audience is malformed.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            token_trust: TokenTrust {
                audience: "fixture".parse()?,
                issuer: "https://partner.fixture".parse()?,
            },
        })
    }
}

#[async_trait]
impl ProvidesEndpoint for PartnerEndpoint {
    async fn provide(&self) -> anyhow::Result<Url> {
        Url::parse("https://partner.fixture/.well-known/jwks.json").map_err(|source| {
            EndpointError::Resolution {
                source: Box::new(source),
            }
            .into()
        })
    }
}

impl DeclaresTokenTrust for PartnerEndpoint {
    fn token_trust(&self) -> &TokenTrust {
        &self.token_trust
    }
}
