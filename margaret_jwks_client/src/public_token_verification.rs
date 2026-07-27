use crate::token_rejection::TokenRejection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicTokenVerification<TClaims> {
    Rejected(TokenRejection),
    Verified(TClaims),
}
