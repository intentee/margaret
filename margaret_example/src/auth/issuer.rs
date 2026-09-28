use margaret::framework::macros::constructor;
use margaret::framework::macros::issues_tokens;
use margaret::framework::macros::singleton;
use margaret::framework::token_issuance::declares_token_issuance::DeclaresTokenIssuance;
use margaret::framework::token_issuance::token_issuance::TokenIssuance;

use crate::auth::issuer_identifier::ISSUER_IDENTIFIER;
use crate::auth::token_audience::TOKEN_AUDIENCE;

#[singleton]
#[issues_tokens]
pub struct Issuer {
    token_issuance: TokenIssuance,
}

impl Issuer {
    /// # Errors
    ///
    /// Returns an error when the issuer identifier or the audience is malformed.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            token_issuance: TokenIssuance {
                audience: TOKEN_AUDIENCE.parse()?,
                issuer: ISSUER_IDENTIFIER.parse()?,
            },
        })
    }
}

impl DeclaresTokenIssuance for Issuer {
    fn token_issuance(&self) -> &TokenIssuance {
        &self.token_issuance
    }
}
