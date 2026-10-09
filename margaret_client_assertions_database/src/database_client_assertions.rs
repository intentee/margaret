use std::sync::Arc;

use async_trait::async_trait;
use futures_util::TryFutureExt as _;

use margaret_accepted_clients::assertion_memory::AssertionMemory;
use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_database::database::Database;
use margaret_model::qualified_framework_table::qualified_framework_table;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::client_assertions_database_error::ClientAssertionsDatabaseError;

fn remember_statement() -> String {
    let table = qualified_framework_table("client_assertions");

    format!(
        "WITH swept AS (DELETE FROM {table} WHERE expires_at <= $4 AND NOT (client_id = $1 AND assertion = $2)) \
         INSERT INTO {table} (client_id, assertion, expires_at) VALUES ($1, $2, $3) \
         ON CONFLICT (client_id, assertion) DO UPDATE SET expires_at = EXCLUDED.expires_at \
         WHERE {table}.expires_at <= $4"
    )
}

pub struct DatabaseClientAssertions {
    database: Arc<Database>,
    remember: String,
}

impl DatabaseClientAssertions {
    #[must_use]
    pub fn create(database: Arc<Database>) -> Self {
        Self {
            database,
            remember: remember_statement(),
        }
    }
}

#[async_trait]
impl RemembersClientAssertions for DatabaseClientAssertions {
    async fn remember_client_assertion(
        &self,
        client_id: &str,
        assertion: TokenDigest,
        expires_at: NumericDate,
        now: NumericDate,
    ) -> anyhow::Result<AssertionMemory> {
        Ok(self
            .database
            .client()
            .map_err(ClientAssertionsDatabaseError::Unavailable)
            .and_then(|client| async move {
                client
                    .execute(
                        &self.remember,
                        &[
                            &client_id,
                            &assertion.as_bytes().as_slice(),
                            &expires_at.seconds_since_epoch(),
                            &now.seconds_since_epoch(),
                        ],
                    )
                    .await
                    .map_err(ClientAssertionsDatabaseError::Remember)
            })
            .await
            .map(|remembered| match remembered {
                0 => AssertionMemory::Seen,
                _ => AssertionMemory::First,
            })?)
    }
}
