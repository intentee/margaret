use margaret::framework::token_digest::token_digest::TokenDigest;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct RememberedAssertion {
    pub assertion: TokenDigest,
    pub client_id: String,
}
