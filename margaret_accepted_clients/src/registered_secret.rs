use margaret_token_digest::token_digest::TokenDigest;

pub(crate) enum RegisteredSecret {
    Digest(TokenDigest),
    Public,
}
