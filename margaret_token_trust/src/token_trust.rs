use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::declares_token_trust::DeclaresTokenTrust;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenTrust {
    pub audience: Audience,
    pub issuer: IssuerIdentifier,
}

impl DeclaresTokenTrust for TokenTrust {
    fn token_trust(&self) -> &TokenTrust {
        self
    }
}
