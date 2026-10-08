use std::sync::Arc;

use async_trait::async_trait;

use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret_jwks_roller::signing_keys_replacement::SigningKeysReplacement;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

use crate::signing_keys_steps::SigningKeysSteps;

pub struct SteppedSigningKeys {
    pub inner: Arc<dyn StoresSigningKeys>,
    pub steps: SigningKeysSteps,
}

#[async_trait]
impl StoresSigningKeys for SteppedSigningKeys {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        self.steps.pass().await;
        self.inner.load_signing_keys().await
    }

    async fn create_signing_keys(
        &self,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysCreation> {
        self.steps.pass().await;
        self.inner.create_signing_keys(revision).await
    }

    async fn replace_signing_keys(
        &self,
        expected: SigningKeysGeneration,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysReplacement> {
        self.steps.pass().await;
        self.inner.replace_signing_keys(expected, revision).await
    }
}
