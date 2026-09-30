use crate::token_digest::TokenDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefreshIssuance {
    Opened(TokenDigest),
    Withheld,
}
