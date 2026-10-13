use margaret_database::database::Database;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys::stored_signing_keys::StoredSigningKeys;

#[derive(Debug, Eq, PartialEq)]
pub enum StoredRevision {
    Absent,
    Stored {
        document: String,
        generation: SigningKeysGeneration,
    },
}

impl StoredRevision {
    /// # Panics
    ///
    /// Panics when the signing keys cannot be loaded.
    pub async fn loaded(database: &Database) -> Self {
        match SigningKeySet::load(database)
            .await
            .expect("the signing keys load")
        {
            StoredSigningKeys::Absent => Self::Absent,
            StoredSigningKeys::Stored(revision) => Self::of(&revision),
        }
    }

    #[must_use]
    pub fn of(
        SigningKeysRevision {
            document,
            generation,
        }: &SigningKeysRevision,
    ) -> Self {
        Self::Stored {
            document: document.expose().to_string(),
            generation: *generation,
        }
    }
}
