use crate::token_issuance::TokenIssuance;

pub trait DeclaresTokenIssuance: Send + Sync {
    fn token_issuance(&self) -> &TokenIssuance;
}
