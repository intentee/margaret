use margaret_token_digest::token_digest::TokenDigest;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct SpentAssertionKey {
    pub(crate) assertion: TokenDigest,
    pub(crate) client_id: String,
}
