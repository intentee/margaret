use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::trusts_oidc_issuer;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret::framework::token_trust::token_trust::TokenTrust;

#[singleton]
#[trusts_oidc_issuer(ci)]
pub struct CiIssuer {
    token_trust: TokenTrust,
}

impl CiIssuer {
    /// # Errors
    ///
    /// Returns an error when the trusted issuer identifier or audience is malformed.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            token_trust: TokenTrust {
                audience: "https://provider.fixture".parse()?,
                issuer: "https://ci.fixture".parse()?,
            },
        })
    }
}

impl DeclaresTokenTrust for CiIssuer {
    fn token_trust(&self) -> &TokenTrust {
        &self.token_trust
    }
}
