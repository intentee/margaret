use margaret::framework::macros::constructor;
use margaret::framework::macros::issues_tokens;
use margaret::framework::macros::singleton;
use margaret::framework::token_issuance::declares_token_issuance::DeclaresTokenIssuance;
use margaret::framework::token_issuance::token_issuance::TokenIssuance;

#[singleton]
#[issues_tokens]
pub struct ProviderIssuer {
    token_issuance: TokenIssuance,
}

impl ProviderIssuer {
    /// # Errors
    ///
    /// Returns an error when the issuer identifier or the audience is malformed.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            token_issuance: TokenIssuance {
                audience: "session".parse()?,
                issuer: "https://provider.fixture".parse()?,
            },
        })
    }
}

impl DeclaresTokenIssuance for ProviderIssuer {
    fn token_issuance(&self) -> &TokenIssuance {
        &self.token_issuance
    }
}
