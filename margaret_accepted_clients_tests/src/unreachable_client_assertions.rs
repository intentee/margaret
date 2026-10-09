use async_trait::async_trait;

use margaret_accepted_clients::assertion_memory::AssertionMemory;
use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::assertion_storage_error::AssertionStorageError;

pub struct UnreachableClientAssertions;

#[async_trait]
impl RemembersClientAssertions for UnreachableClientAssertions {
    async fn remember_client_assertion(
        &self,
        _client_id: &str,
        _assertion: TokenDigest,
        _expires_at: NumericDate,
        _now: NumericDate,
    ) -> anyhow::Result<AssertionMemory> {
        Err(AssertionStorageError::Unreachable.into())
    }
}
