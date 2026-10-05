use margaret_token_digest::token_digest::TokenDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PendingVerdict {
    Approved { code: TokenDigest },
    Denied,
}
