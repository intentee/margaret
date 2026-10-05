use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;
use margaret_token_issuance::token_issuance::TokenIssuance;

pub struct TokenIssuanceDeclaration {
    pub issuance: TokenIssuance,
}

impl DeclaresTokenIssuance for TokenIssuanceDeclaration {
    fn token_issuance(&self) -> &TokenIssuance {
        &self.issuance
    }
}
