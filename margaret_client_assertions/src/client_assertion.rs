use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::upsertion::Upsertion;
use margaret::framework::database::database::Database;
use margaret::framework::macros::model;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

use crate::assertion_memory::AssertionMemory;
use crate::client_assertions_error::ClientAssertionsError;

fn memory(upsertion: Upsertion) -> AssertionMemory {
    match upsertion {
        Upsertion::Kept => AssertionMemory::Seen,
        Upsertion::Written => AssertionMemory::First,
    }
}

#[model(table = "client_assertions")]
#[primary_key(fields = [client_id, assertion])]
pub struct ClientAssertion {
    #[column]
    pub client_id: String,
    #[column(byte_length = 32)]
    pub assertion: Vec<u8>,
    #[column]
    #[index]
    pub expires_at: i64,
}

impl ClientAssertion {
    /// # Errors
    ///
    /// Returns `ClientAssertionsError::Sweep` when the expired assertions cannot be deleted, and
    /// `ClientAssertionsError::Remember` when the assertion cannot be remembered.
    pub async fn remember(
        database: &Database,
        client_id: &str,
        assertion: TokenDigest,
        expires_at: NumericDate,
        now: NumericDate,
    ) -> Result<AssertionMemory, ClientAssertionsError> {
        let now = now.seconds_since_epoch();

        Self::query()
            .expires_at
            .at_most(now)
            .delete(database)
            .await
            .map_err(ClientAssertionsError::Sweep)?;

        let remembered = Self {
            client_id: client_id.to_string(),
            assertion: assertion.as_bytes().to_vec(),
            expires_at: expires_at.seconds_since_epoch(),
        };

        remembered
            .insert()
            .or_update(|columns| columns.expires_at.excluded())
            .when(|existing| existing.expires_at.at_most(now))
            .run(database)
            .await
            .map(memory)
            .map_err(ClientAssertionsError::Remember)
    }
}
