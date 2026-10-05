use margaret_token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret_token_trust::token_trust::TokenTrust;

pub struct TokenTrustDeclaration {
    pub trust: TokenTrust,
}

impl DeclaresTokenTrust for TokenTrustDeclaration {
    fn token_trust(&self) -> &TokenTrust {
        &self.trust
    }
}
