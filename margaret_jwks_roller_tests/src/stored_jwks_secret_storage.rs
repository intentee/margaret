use std::sync::Mutex;
use std::sync::PoisonError;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;

pub struct StoredJwksSecretStorage {
    stored: Mutex<LoadedSecret>,
}

impl StoredJwksSecretStorage {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            stored: Mutex::new(LoadedSecret::Absent),
        }
    }

    #[must_use]
    pub fn seeded(secret: JwksSecret) -> Self {
        Self {
            stored: Mutex::new(LoadedSecret::Present(Box::new(secret))),
        }
    }
}

impl JwksSecretStorage for StoredJwksSecretStorage {
    fn load(&self) -> anyhow::Result<LoadedSecret> {
        let stored = self.stored.lock().unwrap_or_else(PoisonError::into_inner);

        Ok(match &*stored {
            LoadedSecret::Absent => LoadedSecret::Absent,
            LoadedSecret::Present(secret) => LoadedSecret::Present(secret.clone()),
        })
    }

    fn persist(&self, secret: &JwksSecret) -> anyhow::Result<()> {
        let mut stored = self.stored.lock().unwrap_or_else(PoisonError::into_inner);

        *stored = LoadedSecret::Present(Box::new(secret.clone()));

        Ok(())
    }
}
