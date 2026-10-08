use async_trait::async_trait;

use margaret::framework::accepted_clients::assertion_memory::AssertionMemory;
use margaret::framework::accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret::framework::macros::remembers_client_assertions;
use margaret::framework::macros::singleton;
use margaret::framework::registered_claims::numeric_date::NumericDate;
use margaret::framework::token_digest::token_digest::TokenDigest;

#[singleton]
#[remembers_client_assertions]
pub struct AssertionLedger;

#[async_trait]
impl RemembersClientAssertions for AssertionLedger {
    async fn remember_client_assertion(
        &self,
        _client_id: &str,
        _assertion: TokenDigest,
        _expires_at: NumericDate,
        _now: NumericDate,
    ) -> anyhow::Result<AssertionMemory> {
        Ok(AssertionMemory::Seen)
    }
}
