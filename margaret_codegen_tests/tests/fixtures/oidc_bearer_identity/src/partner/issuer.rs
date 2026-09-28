use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::trusts_oidc_issuer;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret::framework::token_trust::token_trust::TokenTrust;

#[singleton]
#[trusts_oidc_issuer(partner)]
pub struct Issuer {
    token_trust: TokenTrust,
}

impl Issuer {
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

impl DeclaresTokenTrust for Issuer {
    fn token_trust(&self) -> &TokenTrust {
        &self.token_trust
    }
}
