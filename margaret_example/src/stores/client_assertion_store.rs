use async_trait::async_trait;
use moka::future::Cache;

use margaret::framework::accepted_clients::assertion_memory::AssertionMemory;
use margaret::framework::accepted_clients::client_assertion_max_lifetime::CLIENT_ASSERTION_MAX_LIFETIME;
use margaret::framework::accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret::framework::macros::constructor;
use margaret::framework::macros::remembers_client_assertions;
use margaret::framework::macros::singleton;
use margaret::framework::registered_claims::numeric_date::NumericDate;
use margaret::framework::token_digest::token_digest::TokenDigest;

use crate::stores::remembered_assertion::RememberedAssertion;

#[singleton]
#[remembers_client_assertions]
pub struct ClientAssertionStore {
    remembered: Cache<RememberedAssertion, NumericDate>,
}

impl ClientAssertionStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            remembered: Cache::builder()
                .time_to_live(CLIENT_ASSERTION_MAX_LIFETIME)
                .build(),
        })
    }
}

#[async_trait]
impl RemembersClientAssertions for ClientAssertionStore {
    async fn remember_client_assertion(
        &self,
        client_id: &str,
        assertion: TokenDigest,
        expires_at: NumericDate,
        now: NumericDate,
    ) -> anyhow::Result<AssertionMemory> {
        let remembered = self
            .remembered
            .entry(RememberedAssertion {
                assertion,
                client_id: client_id.to_string(),
            })
            .or_insert(expires_at)
            .await;

        Ok(if remembered.is_fresh() || *remembered.value() <= now {
            AssertionMemory::First
        } else {
            AssertionMemory::Seen
        })
    }
}
