use async_trait::async_trait;
use tokio::sync::Mutex;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret_jwks_roller::signing_keys_replacement::SigningKeysReplacement;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

use crate::storage_backend_error::StorageBackendError;

struct FixtureSigningKeysState {
    accepted_writes: usize,
    reachable: bool,
    stored: StoredSigningKeys,
}

impl FixtureSigningKeysState {
    fn reachable(&mut self) -> anyhow::Result<&mut Self> {
        if self.reachable {
            Ok(self)
        } else {
            Err(StorageBackendError::Unreachable.into())
        }
    }
}

pub struct FixtureSigningKeys {
    state: Mutex<FixtureSigningKeysState>,
}

impl FixtureSigningKeys {
    #[must_use]
    pub fn empty() -> Self {
        Self::with_contents(StoredSigningKeys::Absent)
    }

    #[must_use]
    pub fn holding(revision: SigningKeysRevision) -> Self {
        Self::with_contents(StoredSigningKeys::Stored(revision))
    }

    /// # Panics
    ///
    /// Panics when the fixture secret cannot be serialized.
    #[must_use]
    pub fn storing(secret: &JwksSecret) -> Self {
        Self::holding(
            SigningKeysRevision::from_secret(secret).expect("the fixture secret serializes"),
        )
    }

    fn with_contents(stored: StoredSigningKeys) -> Self {
        Self {
            state: Mutex::new(FixtureSigningKeysState {
                accepted_writes: 0,
                reachable: true,
                stored,
            }),
        }
    }

    pub async fn accepted_writes(&self) -> usize {
        self.state.lock().await.accepted_writes
    }

    pub async fn break_down(&self) {
        self.state.lock().await.reachable = false;
    }

    pub async fn overwrite(&self, contents: StoredSigningKeys) {
        self.state.lock().await.stored = contents;
    }
}

#[async_trait]
impl StoresSigningKeys for FixtureSigningKeys {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        Ok(self.state.lock().await.reachable()?.stored.clone())
    }

    async fn create_signing_keys(
        &self,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysCreation> {
        let mut guard = self.state.lock().await;
        let state = guard.reachable()?;

        Ok(match state.stored {
            StoredSigningKeys::Absent => {
                state.stored = StoredSigningKeys::Stored(revision.clone());
                state.accepted_writes += 1;

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
        let mut guard = self.state.lock().await;
        let state = guard.reachable()?;

        Ok(match &state.stored {
            StoredSigningKeys::Stored(stored) if stored.generation == expected => {
                state.stored = StoredSigningKeys::Stored(revision.clone());
                state.accepted_writes += 1;

                SigningKeysReplacement::Replaced
            }
            StoredSigningKeys::Absent | StoredSigningKeys::Stored(_) => {
                SigningKeysReplacement::Superseded
            }
        })
    }
}
