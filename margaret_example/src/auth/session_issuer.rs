use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::trusts_oidc_issuer;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret::framework::token_trust::token_trust::TokenTrust;

use crate::auth::issuer_identifier::ISSUER_IDENTIFIER;
use crate::auth::token_audience::TOKEN_AUDIENCE;

#[singleton]
#[trusts_oidc_issuer(auth)]
pub struct SessionIssuer {
    token_trust: TokenTrust,
}

impl SessionIssuer {
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

impl DeclaresTokenTrust for SessionIssuer {
    fn token_trust(&self) -> &TokenTrust {
        &self.token_trust
    }
}
