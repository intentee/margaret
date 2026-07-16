use margaret_jwks_key_gen::jwks_secret::JwksSecret;

use crate::jwks_secret_storage::JwksSecretStorage;
use crate::loaded_secret::LoadedSecret;
use crate::roller_error::RollerError;

pub struct MemoryJwksSecretStorage;

impl JwksSecretStorage for MemoryJwksSecretStorage {
    fn load(&self) -> Result<LoadedSecret, RollerError> {
        Ok(LoadedSecret::Absent)
    }

    fn persist(&self, _secret: &JwksSecret) -> Result<(), RollerError> {
        Ok(())
    }
}
