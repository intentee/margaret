use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Barrier;

use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret_jwks_roller::signing_keys_replacement::SigningKeysReplacement;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

pub struct RacingSigningKeys {
    pub inner: Arc<dyn StoresSigningKeys>,
    pub writers: Arc<Barrier>,
}

#[async_trait]
impl StoresSigningKeys for RacingSigningKeys {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        self.inner.load_signing_keys().await
    }

    async fn create_signing_keys(
        &self,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysCreation> {
        self.writers.wait().await;
        self.inner.create_signing_keys(revision).await
    }

    async fn replace_signing_keys(
        &self,
        expected: SigningKeysGeneration,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysReplacement> {
        self.writers.wait().await;
        self.inner.replace_signing_keys(expected, revision).await
    }
}
