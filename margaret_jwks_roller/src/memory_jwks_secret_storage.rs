use margaret_jwks_keygen::jwks_secret::JwksSecret;

use crate::jwks_secret_storage::JwksSecretStorage;
use crate::loaded_secret::LoadedSecret;

pub struct MemoryJwksSecretStorage;

impl JwksSecretStorage for MemoryJwksSecretStorage {
    fn load(&self) -> anyhow::Result<LoadedSecret> {
        Ok(LoadedSecret::Absent)
    }

    fn persist(&self, _secret: &JwksSecret) -> anyhow::Result<()> {
        Ok(())
    }
}
