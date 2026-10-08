use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::Mutex;

use margaret_accepted_clients::assertion_memory::AssertionMemory;
use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

#[derive(Default)]
pub struct FixtureClientAssertions {
    remembered: Mutex<HashMap<String, HashMap<TokenDigest, NumericDate>>>,
}

#[async_trait]
impl RemembersClientAssertions for FixtureClientAssertions {
    async fn remember_client_assertion(
        &self,
        client_id: &str,
        assertion: TokenDigest,
        expires_at: NumericDate,
        now: NumericDate,
    ) -> anyhow::Result<AssertionMemory> {
        let mut remembered = self.remembered.lock().await;
        let assertions = remembered.entry(client_id.to_string()).or_default();

        Ok(match assertions.get(&assertion) {
            Some(remembered_until) if *remembered_until > now => AssertionMemory::Seen,
            Some(_) | None => {
                assertions.insert(assertion, expires_at);

                AssertionMemory::First
            }
        })
    }
}
