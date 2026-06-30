use crate::security::access_token_claims::AccessTokenClaims;

pub trait SignsTokenClaims: Send + Sync {
    fn sign(&self, claims: &AccessTokenClaims) -> String;
}
