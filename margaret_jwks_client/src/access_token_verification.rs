use margaret_identity_session::claims_rejection::ClaimsRejection;
use margaret_jwks_keygen::token_malformation::TokenMalformation;

pub enum AccessTokenVerification<TClaims> {
    Malformed(TokenMalformation),
    NotReady,
    Rejected(ClaimsRejection),
    SignatureMismatch,
    Verified(TClaims),
}
