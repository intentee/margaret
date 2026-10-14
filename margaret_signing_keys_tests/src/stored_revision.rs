use serde_json::Value;

use margaret_database::database::Database;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys::stored_signing_keys::StoredSigningKeys;

use crate::fixture_signing_keys_encryption_key::fixture_signing_keys_encryption_key;

#[derive(Debug, Eq, PartialEq)]
pub enum StoredRevision {
    Absent,
    Stored {
        document: Value,
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

    /// # Panics
    ///
    /// Panics when the revision does not open under the fixture key.
    #[must_use]
    pub fn of(revision: &SigningKeysRevision) -> Self {
        Self::Stored {
            document: serde_json::to_value(
                revision
                    .clone()
                    .opened(&fixture_signing_keys_encryption_key())
                    .expect("the revision opens under the fixture key"),
            )
            .expect("the opened document serializes"),
            generation: revision.generation,
        }
    }
}
