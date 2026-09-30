use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::trusts_oidc_issuer;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret::framework::token_trust::token_trust::TokenTrust;

use crate::auth::attachments_resource::ATTACHMENTS_RESOURCE;
use crate::auth::issuer_identifier::ISSUER_IDENTIFIER;

#[singleton]
#[trusts_oidc_issuer(margaret)]
pub struct MargaretIssuer {
    token_trust: TokenTrust,
}

impl MargaretIssuer {
    /// # Errors
    ///
    /// Returns an error when the issuer identifier or the attachments resource is malformed.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            token_trust: TokenTrust {
                audience: ATTACHMENTS_RESOURCE.parse()?,
                issuer: ISSUER_IDENTIFIER.parse()?,
            },
        })
    }
}

impl DeclaresTokenTrust for MargaretIssuer {
    fn token_trust(&self) -> &TokenTrust {
        &self.token_trust
    }
}
