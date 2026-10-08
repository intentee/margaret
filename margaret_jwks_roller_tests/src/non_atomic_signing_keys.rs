use async_trait::async_trait;
use tokio::sync::Mutex;

use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret_jwks_roller::signing_keys_replacement::SigningKeysReplacement;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

pub struct NonAtomicSigningKeys {
    stored: Mutex<StoredSigningKeys>,
}

impl NonAtomicSigningKeys {
    #[must_use]
    pub fn new(stored: StoredSigningKeys) -> Self {
        Self {
            stored: Mutex::new(stored),
        }
    }
}

#[async_trait]
impl StoresSigningKeys for NonAtomicSigningKeys {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        Ok(self.stored.lock().await.clone())
    }

    async fn create_signing_keys(
        &self,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysCreation> {
        *self.stored.lock().await = StoredSigningKeys::Stored(revision.clone());

        Ok(SigningKeysCreation::Created)
    }

    async fn replace_signing_keys(
        &self,
        _expected: SigningKeysGeneration,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysReplacement> {
        *self.stored.lock().await = StoredSigningKeys::Stored(revision.clone());

        Ok(SigningKeysReplacement::Replaced)
    }
}
