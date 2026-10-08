use async_trait::async_trait;

use margaret::framework::jwks_roller::signing_keys_document::SigningKeysDocument;
use margaret::framework::jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret::framework::jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret::framework::macros::singleton;
use margaret::framework::macros::stores_signing_keys;

#[singleton]
#[stores_signing_keys]
pub struct SigningKeyStore;

#[async_trait]
impl StoresSigningKeys for SigningKeyStore {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        Ok(StoredSigningKeys::Absent)
    }

    async fn store_signing_keys(&self, _document: &SigningKeysDocument) -> anyhow::Result<()> {
        Ok(())
    }
}
