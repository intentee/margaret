use async_trait::async_trait;
use tokio::sync::Mutex;

use margaret::framework::jwks_roller::signing_keys_document::SigningKeysDocument;
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

    async fn store_signing_keys(&self, document: &SigningKeysDocument) -> anyhow::Result<()> {
        *self.stored.lock().await = StoredSigningKeys::Stored(document.clone());

        Ok(())
    }
}
