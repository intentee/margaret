use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

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
    /// Panics when the store cannot be reached.
    pub async fn loaded(store: &dyn StoresSigningKeys) -> Self {
        match store
            .load_signing_keys()
            .await
            .expect("the store loads its signing keys")
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
            document: document.json().to_string(),
            generation: *generation,
        }
    }
}
