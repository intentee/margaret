use margaret_jwks_keygen::token_malformation::TokenMalformation;

pub enum AccessTokenVerification<TClaims> {
    AudienceMismatch,
    Expired,
    IssuerMismatch,
    Malformed(TokenMalformation),
    NotReady,
    SignatureMismatch,
    Verified(TClaims),
}
