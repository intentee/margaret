use async_trait::async_trait;
use tokio::sync::Mutex;

use margaret::framework::jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret::framework::jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret::framework::jwks_roller::signing_keys_replacement::SigningKeysReplacement;
use margaret::framework::jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret::framework::jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret::framework::jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::stores_signing_keys;

#[singleton]
#[stores_signing_keys]
pub struct SigningKeyStore {
    stored: Mutex<StoredSigningKeys>,
}

impl SigningKeyStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            stored: Mutex::new(StoredSigningKeys::Absent),
        })
    }
}

#[async_trait]
impl StoresSigningKeys for SigningKeyStore {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        Ok(self.stored.lock().await.clone())
    }

    async fn create_signing_keys(
        &self,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysCreation> {
        let mut stored = self.stored.lock().await;

        Ok(match *stored {
            StoredSigningKeys::Absent => {
                *stored = StoredSigningKeys::Stored(revision.clone());

                SigningKeysCreation::Created
            }
            StoredSigningKeys::Stored(_) => SigningKeysCreation::AlreadyCreated,
        })
    }

    async fn replace_signing_keys(
        &self,
        expected: SigningKeysGeneration,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysReplacement> {
        let mut stored = self.stored.lock().await;

        Ok(match &*stored {
            StoredSigningKeys::Stored(current) if current.generation == expected => {
                *stored = StoredSigningKeys::Stored(revision.clone());

                SigningKeysReplacement::Replaced
            }
            StoredSigningKeys::Absent | StoredSigningKeys::Stored(_) => {
                SigningKeysReplacement::Superseded
            }
        })
    }
}
