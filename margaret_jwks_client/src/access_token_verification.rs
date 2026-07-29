use margaret_jwks_keygen::token_malformation::TokenMalformation;

pub enum AccessTokenVerification<TClaims> {
    Expired,
    Malformed(TokenMalformation),
    NotReady,
    SignatureMismatch,
    Verified(TClaims),
}
