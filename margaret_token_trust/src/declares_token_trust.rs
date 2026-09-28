use crate::token_trust::TokenTrust;

pub trait DeclaresTokenTrust: Send + Sync {
    fn token_trust(&self) -> &TokenTrust;
}
