use margaret_token_digest::random_token::random_token;
use margaret_token_digest::token_digest::TokenDigest;

#[must_use]
pub fn contract_token() -> TokenDigest {
    TokenDigest::of(&random_token())
}
