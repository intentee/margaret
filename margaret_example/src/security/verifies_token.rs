use crate::security::access_token_claims::AccessTokenClaims;

pub trait VerifiesToken: Send + Sync {
    fn verify(&self, token: &str) -> Option<AccessTokenClaims>;
}
