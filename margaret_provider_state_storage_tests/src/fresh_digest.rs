use uuid::Uuid;

use margaret_provider_state_storage::token_digest::TokenDigest;

#[must_use]
pub fn fresh_digest() -> TokenDigest {
    TokenDigest::of(&Uuid::new_v4().to_string())
}
