use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::trusts_oidc_issuer;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret::framework::token_trust::token_trust::TokenTrust;

#[singleton]
#[trusts_oidc_issuer(partner)]
pub struct PartnerIssuer {
    token_trust: TokenTrust,
}

impl PartnerIssuer {
    /// # Errors
    ///
    /// Returns an error when the trusted issuer identifier or audience is malformed.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            token_trust: TokenTrust {
                audience: "artifacts".parse()?,
                issuer: "https://partner.fixture".parse()?,
            },
        })
    }
}

impl DeclaresTokenTrust for PartnerIssuer {
    fn token_trust(&self) -> &TokenTrust {
        &self.token_trust
    }
}
