use crate::token_malformation::TokenMalformation;

pub enum JwksSecretVerificationResult<TClaims> {
    Invalid,
    Malformed(TokenMalformation),
    SignedWithCurrent(TClaims),
    SignedWithPrevious(TClaims),
}
