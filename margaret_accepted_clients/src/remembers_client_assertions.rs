use async_trait::async_trait;

use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::assertion_memory::AssertionMemory;

#[async_trait]
pub trait RemembersClientAssertions: Send + Sync {
    /// Remembers the assertion of the client until it expires, reporting it as first exactly once
    /// across every instance while it is remembered.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn remember_client_assertion(
        &self,
        client_id: &str,
        assertion: TokenDigest,
        expires_at: NumericDate,
        now: NumericDate,
    ) -> anyhow::Result<AssertionMemory>;
}
